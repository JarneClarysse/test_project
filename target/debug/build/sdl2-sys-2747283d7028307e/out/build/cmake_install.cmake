# Install script for directory: /home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/libSDL2.a")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/libSDL2main.a")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL2" TYPE FILE FILES
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_assert.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_atomic.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_audio.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_bits.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_blendmode.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_clipboard.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_android.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_iphoneos.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_macosx.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_minimal.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_pandora.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_psp.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_windows.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_winrt.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_config_wiz.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_copying.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_cpuinfo.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_egl.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_endian.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_error.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_events.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_filesystem.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_gamecontroller.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_gesture.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_haptic.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_hints.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_joystick.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_keyboard.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_keycode.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_loadso.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_log.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_main.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_messagebox.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_mouse.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_mutex.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_name.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengl.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengl_glext.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles2.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles2_gl2.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles2_gl2ext.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles2_gl2platform.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_opengles2_khrplatform.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_pixels.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_platform.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_power.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_quit.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_rect.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_render.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_revision.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_rwops.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_scancode.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_shape.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_stdinc.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_surface.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_system.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_syswm.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_assert.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_common.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_compare.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_crc32.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_font.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_fuzzer.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_harness.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_images.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_log.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_md5.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_test_random.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_thread.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_timer.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_touch.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_types.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_version.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/SDL_video.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/begin_code.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/include/close_code.h"
    "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/include/SDL_config.h"
    )
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/sdl2.pc")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/sdl2-config")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/share/aclocal/sdl2.m4")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/share/aclocal" TYPE FILE FILES "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/SDL2-2.0.5/sdl2.m4")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
