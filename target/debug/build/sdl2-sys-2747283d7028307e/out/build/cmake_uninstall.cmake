if (NOT EXISTS "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/install_manifest.txt")
    message(FATAL_ERROR "Cannot find install manifest: \"/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/install_manifest.txt\"")
endif(NOT EXISTS "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/install_manifest.txt")

file(READ "/home/pi/Veilige_soft_test/test_project/test_project/target/debug/build/sdl2-sys-2747283d7028307e/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach (file ${files})
    message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
    execute_process(
        COMMAND /usr/bin/cmake -E remove "$ENV{DESTDIR}${file}"
        OUTPUT_VARIABLE rm_out
        RESULT_VARIABLE rm_retval
    )
    if(NOT ${rm_retval} EQUAL 0)
        message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif (NOT ${rm_retval} EQUAL 0)
endforeach(file)

