#include "sys/wait.h"
#include "stdio.h"
#include <stdlib.h>
#include "unistd.h"

int main(int argc, char *argv[]) {
    char *arguments[] = {argv[1], argv[2], NULL};
    int status;
    int pid = fork();
    if(pid == 0){
        execvp(argv[1], arguments);
        exit(1);
    }
    else{
        waitpid(pid, &status, 0);
        if(status == 0){
            printf("Success! \n");
        }
        else{
            printf("Failed, exit code = %d \n", status);
        }
    }
    return 0;
}
