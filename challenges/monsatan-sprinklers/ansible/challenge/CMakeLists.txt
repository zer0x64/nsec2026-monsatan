cmake_minimum_required(VERSION 3.10)

project(monsatan-sprinklers C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(OpenSSL REQUIRED)
find_package(Threads REQUIRED)

# Server-side executable
add_executable(${CMAKE_PROJECT_NAME} src/main.c src/auth.c src/cmd.c src/robot.c src/scada.c src/threadpool.c)
#target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE "include")
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE OpenSSL::SSL OpenSSL::Crypto Threads::Threads)

if(USE_LOCALHOST_PORT)
    target_compile_definitions(${CMAKE_PROJECT_NAME} PRIVATE USE_LOCALHOST_PORT)
endif()

# Client-side executable
add_executable("sprinklerbotctl" src/cli.c)
#target_include_directories("sprinklerbotctl" PRIVATE "include")
target_link_libraries("sprinklerbotctl" PRIVATE OpenSSL::SSL OpenSSL::Crypto)

if(USE_LOCALHOST_PORT)
    target_compile_definitions("sprinklerbotctl" PRIVATE USE_LOCALHOST_PORT)
endif()
