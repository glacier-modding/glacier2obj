from subprocess import Popen, PIPE
from threading import Thread
import time


def get_input(input_text, i):
    print(input_text, file=i, flush=True)


def print_output(o, queue, start):
    output = o.read()
    print(output)
    end = time.time()
    exec_time = end - start
    print("Execution time:" + str(exec_time))
    # handle_output(output, p, queue)


def get_depends_for_scenario(scenario_hashcode):
    p = open_file(scenario_hashcode)

    queue = [scenario_hashcode]
    pop_queue(p, queue)


def pop_queue(p, queue):
    if len(queue) == 0:
        print("Done.")
        return
    hashcode = queue.pop(0)
    input_text = hash_depends_command(hashcode)
    print("Getting depends for hash: " + hashcode + "...")
    start = time.time()
    thread = Thread(target=get_input, args=(input_text, p.stdin,))
    thread.start()
    thread2 = Thread(target=print_output, args=(p.stdout, queue, start))
    thread2.start()


def handle_output(output, p, queue):
    for line in output:
        if "TEMP" in line or "ALOC" in line or "PRIM" in line:
            line_parts = line.split("Hash file/resource: ")
            hash_parts = line_parts[1].split(".")
            new_hashcode = hash_parts[0]
            print("Adding hashcode of type " + hash_parts[1] + " to queue: " + new_hashcode)
            queue.append(new_hashcode)
        if "has reverse dependencies" in line:
            break
    # pop_queue(p, queue)


def hash_depends_command(hashcode):
    input_text = r"-hash_depends D:\Glacier2ObjFiles -filter " + hashcode
    return input_text


def open_file(scenario_hashcode):
    rpkg_dir = r".\rpkg"
    rpkg_exe_location = r".\rpkg\rpkg-cli.exe"
    print(f'Generating OBJ from Scenario ' + scenario_hashcode)
    p = Popen([rpkg_exe_location, "-i"], stdin=PIPE, stdout=PIPE, text=True, cwd=rpkg_dir, universal_newlines=True)
    return p


if __name__ == '__main__':
    get_depends_for_scenario("00276FD0DE7FA8B5")
