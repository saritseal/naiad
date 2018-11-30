loop() {
    for dir in `ls -d */|grep -v target`
    do
        cd $dir
        cargo $1
        cd ..
    done
}


for i in $@
do 
    case $i in
        -c|--clean)
            loop clean
            shift
            ;;
        -b|--build)
            loop build
            shift
            ;;
        -t|--test)
            loop test
            shift
            ;;
        -e|--executable)   
            loop rustc
            shift
            ;;
        -h|--help)
            echo "pass -c/--clean for removing the target directories, pass -b/--build to build the project ;"            
            shift
            ;;
    esac
done

#set -- "${POSITIONAL[@]}" #