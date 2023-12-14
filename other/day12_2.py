from functools import cache


def parse(content: str, copies: int = 1) -> list[tuple[str, list[int]]]:
    parsed = []
    for line in content.splitlines():
        data, groups = line.split(" ")
        groups = tuple(int(num) for num in groups.split(","))
        # make copies
        data = "?".join([data] * copies)
        groups *= copies
        parsed.append((data, groups))
    return parsed


def skip(data: str, groups: tuple[int]) -> int:
    return solve(data[1:], groups)


def lay(data: str, groups: tuple[int]) -> int:
    # lay means we start planting the groups
    if groups == ():
        # we cannot lay anything if there is no more group
        return 0
    curr_group = groups[0]
    if len(data) < curr_group:
        # we cannot lay if not enough space
        return 0
    else:
        if "." in data[:curr_group]:
            # cant lay because '.' underneath
            return 0
        if len(data) == curr_group:
            # special case at the very end
            return solve(data[curr_group:], groups[1:])
        else:
            # because this is not the very end, the +1 after that must NOT be #
            # because if . we can fullfill current group, if ? we assume .
            if data[curr_group] == "#":
                return 0
            else:
                return solve(data[curr_group + 1 :], groups[1:])


@cache
def solve(data: str, groups: tuple[int]) -> int:
    # print(data, groups)
    if data == "" and groups == ():
        # we reached valid finish state
        return 1
    elif data == "" and groups != ():
        # invalid state, prune early
        return 0

    # at this point, your data is not empty, but groups can be either
    # for example "......" () is valid
    # "...##.." (2,) is valid too
    # so group emptiness cannot be used for detecting validness
    curr = data[0]
    total_so_far = 0
    if curr == ".":
        # if we find '.', the only logical way is to skip
        total_so_far += skip(data, groups)
    elif curr == "#":
        # if we find '#', we must lay, because if we skip # then # will be a stray #
        total_so_far += lay(data, groups)
    elif curr == "?":
        # we can choose to lay and skip
        total_so_far += skip(data, groups)
        total_so_far += lay(data, groups)
    else:
        raise RuntimeError(f"impossible case, bad input: {curr}")
    return total_so_far


def main():
    with open("inputs/day12.txt", "r") as f:
        content = f.read()
    parsed = parse(content, copies=5)
    total = 0
    for data, groups in parsed:
        # print(data)
        # print(groups)
        total += solve(data, groups)
    print(f"[PYTHON] day 12 part 2: {total}")


if __name__ == "__main__":
    main()
