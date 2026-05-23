#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

#include <pthread.h>

#include "threadpool.h"

typedef struct {
    pthread_mutex_t lock;
    pthread_cond_t notify;
    pthread_t *threads;
    threadpool_task_t *queue;
    int threadCount;
    int queueSize;
    int head;
    int tail;
    int count;
    bool shutdown;
} threadpool_t;

static threadpool_t *gThreadpool = NULL;

static threadpool_task_t dequeue() {
    threadpool_task_t task;

    pthread_mutex_lock(&gThreadpool->lock);

    // Wait until there's work OR we are shutting down
    while (gThreadpool->count == 0 && !gThreadpool->shutdown) {
        pthread_cond_wait(&gThreadpool->notify, &gThreadpool->lock);
    }

    // If we are shutting down and no tasks remain, return a "null" task
    if (gThreadpool->shutdown && gThreadpool->count <= 0) {
        task.function = NULL;
    } else {
        task = gThreadpool->queue[gThreadpool->head];
        gThreadpool->head = (gThreadpool->head + 1) % gThreadpool->queueSize;
        gThreadpool->count--;
    }

    pthread_mutex_unlock(&gThreadpool->lock);
    return task;
}

void* workerThread(void*) {
    while(true) {
        threadpool_task_t task = dequeue();
        if (task.function == NULL)
            break;

        task.function(task.sock);
    }

    return NULL;
}

void threadpoolCreate(int numThreads) {
    gThreadpool = malloc(sizeof(threadpool_t));
    gThreadpool->threadCount = numThreads;
    gThreadpool->queueSize = numThreads;
    gThreadpool->head = 0;
    gThreadpool->tail = 0;
    gThreadpool->shutdown = false;

    gThreadpool->threads = malloc(sizeof(pthread_t) * gThreadpool->threadCount);
    gThreadpool->queue = malloc(sizeof(threadpool_task_t) * gThreadpool->queueSize);

    pthread_mutex_init(&gThreadpool->lock, NULL);
    pthread_cond_init(&gThreadpool->notify, NULL);

    for (int i = 0; i < gThreadpool->threadCount; i++) {
        pthread_create(&gThreadpool->threads[i], NULL, workerThread, NULL);
    }
}

void threadpoolDestroy() {
    if (gThreadpool == NULL) return;

    pthread_mutex_lock(&gThreadpool->lock);
    gThreadpool->shutdown = true;

    // Wake up all worker threads so they can check the shutdown flag
    pthread_cond_broadcast(&gThreadpool->notify);
    pthread_mutex_unlock(&gThreadpool->lock);

    // Wait for all threads to finish their current task and exit
    for (int i = 0; i < gThreadpool->threadCount; i++) {
        pthread_join(gThreadpool->threads[i], NULL);
    }

    // Clean up memory
    free(gThreadpool->threads);
    free(gThreadpool->queue);
    pthread_mutex_destroy(&gThreadpool->lock);
    pthread_cond_destroy(&gThreadpool->notify);
    free(gThreadpool);
}

bool threadpoolEnqueue(threadpool_task_t task) {
    pthread_mutex_lock(&gThreadpool->lock);

    if (gThreadpool->count >= gThreadpool->queueSize) {
        pthread_mutex_unlock(&gThreadpool->lock);
        return false;
    }

    gThreadpool->queue[gThreadpool->tail] = task;
    gThreadpool->tail = (gThreadpool->tail + 1) % gThreadpool->queueSize;
    gThreadpool->count++;

    pthread_cond_signal(&gThreadpool->notify);
    pthread_mutex_unlock(&gThreadpool->lock);
    return true;
}
