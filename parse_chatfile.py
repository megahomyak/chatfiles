def parse(chatfile):
    chatfile = chatfile.replace("\r", "")
    chatfile_lines = chatfile.split("\n")
    if len(chatfile_lines) > 0 and chatfile_lines[-1] == "":
        chatfile_lines.pop()
    result_lines = []
    for line in chatfile_lines:
        if line.startswith("\\\\"):
            result_lines.append(("regular", line[1:]))
        elif line.startswith("\\"):
            result_lines.append(("special", line[1:]))
        else:
            result_lines.append(("regular", line))
    return result_lines

for line in parse(open("chatfile_example.txt").read()):
    print(line)
