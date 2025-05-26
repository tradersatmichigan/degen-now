#include <iostream>
#include <ostream>
#include <string>

static void printHelp() {
  std::cout << "must specify starting stack size : ./blackjack <STACK_SIZE>\n";
}

int main(int argc, char **argv) {

  if (argc != 2) {
    printHelp();
    return 1;
  }

  unsigned stack = std::stoi(argv[1]);

  std::cout << stack << std::endl;
}
