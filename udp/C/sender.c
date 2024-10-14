#include <stdio.h>
#include <sys/socket.h>
#include <netinet/ip.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <arpa/inet.h>

#define BUFF_LEN 1024
#define DEFAULT_PORT 2000
#define INF 0xffffff

extern int errno;

void send_udp_msg(int sockfd, struct sockaddr* dst, int n) {
    char buff[BUFF_LEN] = "Hello UDP!\n";
    socklen_t len;
    int ret;

    while (n--) {
        len = sizeof(*dst);
        printf("We send: %s\n", buff);
        ret = sendto(sockfd, buff, strlen(buff), 0, dst, len);
        if (ret < 0) {
            fprintf(stderr, "Error on sending: %s\n", strerror(errno));
            exit(-1);
        }
        sleep(1);
    }
}


int main(int argc, char** argv) {
    int send_sockfd, times = INF;
    struct sockaddr_in dst;

    send_sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (send_sockfd < 0) {
        fprintf(stderr, "Error on creating socket: %s\n", strerror(errno));
        exit(-1);
    }

    memset(&dst, 0, sizeof(dst));
    dst.sin_family = AF_INET;
    dst.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    dst.sin_port = htons(DEFAULT_PORT);

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            printf("Send specified message to an IP address.\n\
Argument:\n\t-a: Specify an IPv4 address, \tdefault is \"127.0.0.1\".\n\
\t-p: Specify a port, \t\tdefault is \"2000\".\n\
\t-n: Times you send a message, \tdefault is \"%d\".\n", INF);
            return 0;
        } 
        else if (strcmp(argv[i], "-p") == 0) {
            dst.sin_port = htons(atoi(argv[++i]));
        } 
        else if (strcmp(argv[i], "-a") == 0) {
            dst.sin_addr.s_addr = inet_addr(argv[++i]);
        } 
        else if (strcmp(argv[i], "-n") == 0) {
            times = atoi(argv[++i]);
        } else {
            printf("You entered a wrong argument! Use \"-h\" or \"--help\" to get help.\n");
            exit(-1);
        }
    }

    send_udp_msg(send_sockfd, (struct sockaddr*)&dst, times);

    close(send_sockfd);
    return 0;
}