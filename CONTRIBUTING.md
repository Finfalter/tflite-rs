tflite now includes the tensorflow git repository as a submodule
along with the results of calling `download_dependencies.sh`.
This reduces the number of build steps from ~260 to less than 100. 

If the version of tensorflow is ever updated, `submodules/cleanup-downloads.sh` 
should also be updated if necessary. It is called from within the build script,
`build.rs`. Its purpose is to remove most of the files that are obviously not
needed.
