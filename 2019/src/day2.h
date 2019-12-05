#pragma once

#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "intcode.h"

namespace aoc {

class day2 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto program_output = part_one(program);

    std::cout << "Part one: " << program_output[0] << std::endl;

    int sum = part_two(program);

    std::cout << "Part two: " << sum << std::endl;
  }

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static std::vector<int> part_one(std::vector<int> program) {
    program[1] = 12;
    program[2] = 2;
    run_program(program);

    return program;
  }

  static void run_program(std::vector<int> &program) {
    int program_pointer = 0;

    while (program[program_pointer] != 99) {
      program_pointer = intcode::run_instruction(program, program_pointer);
    }
  }

  static int part_two(std::vector<int> program) {
    int sought_output = 19690720;
    for (int noun = 0; noun < 99; noun++) {
      for (int verb = 0; verb < 99; verb++) {
        std::vector<int> new_program(program.begin(), program.end());
        new_program[1] = noun;
        new_program[2] = verb;
        run_program(new_program);

        if (new_program[0] == sought_output) {
          return 100 * noun + verb;
        }
      }
    }
    return -1;
  }

};
} // namespace aoc
