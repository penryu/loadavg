cmake_minimum_required (VERSION 3.4.3)

project (loadavg
  VERSION 1.1.0
  LANGUAGES C)

include_directories(
  ${loadavg_SOURCE_DIR}/include
  "${PROJECT_BINARY_DIR}")

set (CMAKE_C_EXTENSIONS 1)      # allow GNU extensions
set (CMAKE_C_FLAGS "-Werror -Wall -Wextra -Wconversion -pedantic -fno-builtin")
set (CMAKE_C_FLAGS_DEBUG "-g -O0")
set (CMAKE_C_FLAGS_RELEASE "-O2")
set (CMAKE_C_STANDARD 99)

add_executable(loadavg loadavg.c)

install (TARGETS loadavg DESTINATION bin)
