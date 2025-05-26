#include <cstdio>
#include <cstring>

struct Options {
  bool host = false;
  const char *host_ip = nullptr;
  const char *host_port = nullptr;
};

Options options;

void host() {
  printf("HOSTING");
}

void join() {
  printf("JOINING");
}

int main(int argc, char **argv) {

  for (int i = 1; i < argc; ++i) {
    if      (!strcmp(argv[i], "-h"))  options.host      = true;
    else if (!strcmp(argv[i], "-p"))  options.host_port = argv[i + 1];
    else if (!strcmp(argv[i], "-a")) options.host_ip   = argv[i + 1];
  }

  if (options.host) {
    host();
  } else { // join
    join();
  }

  return 0;
}
