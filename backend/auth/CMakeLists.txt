cmake_minimum_required(VERSION 3.7)

project(auth)

set(CMAKE_CXX_STANDARD 17)

file(GLOB SRC
  src/main.cpp
  src/controllers/*.cpp
)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
configure_file(${CMAKE_SOURCE_DIR}/.env ${CMAKE_BINARY_DIR}/bin/.env COPYONLY)

set(EXTERNAL_INSTALL_LOCATION ${CMAKE_BINARY_DIR}/external)

set(JWT_INSTALL_LOCATION ${EXTERNAL_INSTALL_LOCATION}/jwt-cpp)
set(DROGON_INSTALL_LOCATION ${EXTERNAL_INSTALL_LOCATION}/drogon)
set(FMT_INSTALL_LOCATION ${EXTERNAL_INSTALL_LOCATION}/fmt)

include(FetchContent)

if(NOT TARGET jwt-cpp)
    FetchContent_Declare(
        jwt-cpp
        GIT_REPOSITORY https://github.com/Thalhammer/jwt-cpp.git
        GIT_TAG master
    )
    set(JWT_BUILD_EXAMPLES OFF CACHE BOOL "Configure CMake to build examples (or not)" FORCE)
    FetchContent_MakeAvailable(jwt-cpp)
endif()

if(NOT TARGET drogon)
    FetchContent_Declare(
        drogon
        GIT_REPOSITORY https://github.com/drogonframework/drogon.git
        GIT_TAG master
    )
    set(BUILD_EXAMPLES OFF CACHE BOOL "Build examples" FORCE)
    set(BUILD_CTL OFF CACHE BOOL "Build drogon_ctl" FORCE)
    set(BUILD_ORM OFF CACHE BOOL "Build orm" FORCE)
    FetchContent_MakeAvailable(drogon)
endif()

if(NOT TARGET spdlog)
    FetchContent_Declare(
        spdlog
        GIT_REPOSITORY https://github.com/gabime/spdlog.git
        GIT_TAG v1.x
    )
    FetchContent_MakeAvailable(spdlog)
endif()

if(NOT TARGET nlohmann_json)
    FetchContent_Declare(
        nlohmann_json
        URL
        https://github.com/nlohmann/json/releases/download/v3.11.3/json.tar.xz
    )
    FetchContent_MakeAvailable(nlohmann_json)
endif()

if(NOT TARGET fmt)
    FetchContent_Declare(
        fmt
        GIT_REPOSITORY https://github.com/fmtlib/fmt.git
        GIT_TAG master
    )
    FetchContent_MakeAvailable(fmt)
endif()

set(jwt-cpp_INCLUDE_DIRS ${jwt-cpp_SOURCE_DIR}/include)
set(drogon_INCLUDE_DIRS ${drogon_SOURCE_DIR}/lib/inc)
set(spdlog_INCLUDE_DIRS ${spdlog_SOURCE_DIR}/include)
set(fmt_INCLUDE_DIRS ${fmt_SOURCE_DIR}/include)

include_directories(${CMAKE_SOURCE_DIR}/include ${jwt-cpp_INCLUDE_DIRS} ${drogon_INCLUDE_DIRS} ${spdlog_INCLUDE_DIRS} ${fmt_INCLUDE_DIRS})
link_directories(${CMAKE_SOURCE_DIR}/lib)

add_executable(users ${SRC})
target_include_directories(users PRIVATE ${jwt-cpp_INCLUDE_DIRS} ${drogon_INCLUDE_DIRS} ${spdlog_INCLUDE_DIRS} ${fmt_INCLUDE_DIRS})
target_link_libraries(users PRIVATE drogon jwt-cpp spdlog nlohmann_json::nlohmann_json fmt)

set_target_properties(jwt-cpp PROPERTIES INSTALL_RPATH ${JWT_INSTALL_LOCATION})
set_target_properties(users PROPERTIES INSTALL_RPATH ${DROGON_INSTALL_LOCATION})

install(TARGETS users DESTINATION ${CMAKE_BINARY_DIR}/bin)
install(DIRECTORY ${JWT_INSTALL_LOCATION}/include/ DESTINATION ${CMAKE_BINARY_DIR}/include)
install(DIRECTORY ${DROGON_INSTALL_LOCATION}/lib/inc/ DESTINATION ${CMAKE_BINARY_DIR}/include)
install(DIRECTORY ${spdlog_INSTALL_LOCATION}/include/ DESTINATION ${CMAKE_BINARY_DIR}/include)
install(DIRECTORY ${fmt_INSTALL_LOCATION}/include/ DESTINATION ${CMAKE_BINARY_DIR}/include)
install(FILES ${CMAKE_BINARY_DIR}/lib/libjwt-cpp.a DESTINATION ${CMAKE_BINARY_DIR}/lib)
install(FILES ${CMAKE_BINARY_DIR}/lib/libdrogon.a DESTINATION ${CMAKE_BINARY_DIR}/lib)
install(FILES ${CMAKE_BINARY_DIR}/lib/libfmt.a DESTINATION ${CMAKE_BINARY_DIR}/lib)