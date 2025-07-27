def parse(chatfile):
    chatfile = chatfile.replace("\r", "")
    result_lines = []
    for line in chatfile.split("\n"):
        if line.startswith("\\\\"):
            result_lines.append(("regular", line[1:]))
        elif line.startswith("\\"):
            result_lines.append(("special", line[1:]))
        else:
            result_lines.append(("regular", line))
    if len(chatfile) > 0 and chatfile[-1] == "":
        chatfile.pop()
    return result_lines

for line in parse(open("chatfile_example.txt").read()):
    print(line)
