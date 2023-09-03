cmake_minimum_required(VERSION 3.20)
project(requests)

cmake_policy(SET CMP0091 NEW)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_BINARY_DIR};${CMAKE_SOURCE_DIR}/lib/cmake-modules")

if(NOT EXISTS "${CMAKE_BINARY_DIR}/conan.cmake")
	message(STATUS "Downloading conan.cmake from https://github.com/conan-io/cmake-conan")
	file(DOWNLOAD "https://raw.githubusercontent.com/conan-io/cmake-conan/0.18.1/conan.cmake"
								"${CMAKE_BINARY_DIR}/conan.cmake"
								TLS_VERIFY ON)
endif()

include("${CMAKE_BINARY_DIR}/conan.cmake")

# TODO: conan_cmake_run is deprecated
conan_cmake_run(REQUIRES cpprestsdk/2.10.18
							 BASIC_SETUP BUILD missing
							 SETTINGS arch=x86)

if (MSVC)

	add_link_options(/INCREMENTAL:NO /NODEFAULTLIB:MSVCRT)
	# link runtime statically
	set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>DLL")
	set(MSVC_RUNTIME_LIBRARY_CONFIG "/MD")

elseif (UNIX)

	add_compile_options(
		-m32
		-fvisibility=hidden
		# Possibly one of them can be enabled: -std=gnu++98, -std=c++14
		# $<$<CXX_COMPILER_ID:GNU>:-std=c++14>
		# TODO: Uncomment to enable -pedantic
		#$<$<CXX_COMPILER_ID:GNU>:-pedantic>
	)

	set_property(GLOBAL PROPERTY FIND_LIBRARY_USE_LIB64_PATHS OFF)

	add_link_options(
		"-static-libgcc"
		"-static-libstdc++"
	)

	set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-rpath='$ORIGIN'")

endif()

# Output
set(CMAKE_BINARY_DIR ${CMAKE_SOURCE_DIR}/build/plugins)
set(LIBRARY_OUTPUT_PATH ${CMAKE_SOURCE_DIR}/build/plugins)

set(CMAKE_BUILD_TYPE Release)
set(BUILD_SHARED_LIBS OFF)
set(LOGGING_ENABLED OFF)
set(BUILD_EXAMPLES OFF)
set(BUILD_TESTS OFF)

add_subdirectory(src)
