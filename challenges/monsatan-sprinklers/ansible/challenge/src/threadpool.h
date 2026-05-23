#ifndef THREADPOOL_H
#define THREADPOOL_H

typedef void (*threadpool_func_t)(int sock);

typedef struct {
    threadpool_func_t function;
    int sock;
} threadpool_task_t;

void threadpoolCreate(int numThreads);
void threadpoolDestroy();
bool threadpoolEnqueue(threadpool_task_t task);

#endif
