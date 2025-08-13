# Specify the minimum version.
cmake_minimum_required(VERSION 3.9)

# Specify the project info.
project(mylib VERSION 1.0.0 DESCRIPTION "Shared library test")

# Declare the library target.
add_library(${PROJECT_NAME} STATIC
    mylib.c
)

include_directories(include)

# Set the version property.
set_target_properties(${PROJECT_NAME} PROPERTIES VERSION ${PROJECT_VERSION})

# Set the shared object version property to the project's major version.
set_target_properties(${PROJECT_NAME} PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})

# Set the public header property to the one with the actual API.
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER mylib.h)