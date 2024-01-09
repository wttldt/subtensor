CLIPPY_ARGS=""

for arg
do

    if [ "$arg" == "--apply" ]
    then

        CLIPPY_ARGS="$CLIPPY_ARGS --fix";

    fi

done

cargo \
    clippy ${CLIPPY_ARGS} -- \
    -A clippy::needless_return