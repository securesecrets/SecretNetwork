#getting documentation to run
## Getting Docker container to run :

```docker container run -it <container-id> /bin/bash```

## In Docker Container:
If you run into error

```
error obtaining VCS status: exit status 128
in 
make: *** [Makefile:140: build_local_no_rust] Error 1
```

```git config --global --add safe.directory <PATH to Secret Network directory>```

### Getting local network setup
checkout /scripts/start-node.sh in the directory and run it
Update