# LibertyC2
Esempio di C2 server in rust con multi-connessione e annessa shell.py di prova e server.py per evitare di fare cargo build > run

## Installazione su Debian
apt-get install tor

edit /etc/tor/torrc

HiddenServiceDir /var/lib/tor/hidden_service/

HiddenServicePort 80 127.0.0.1:1234

e siamo pronti per far partire il C2 server, poi per prendere il link onion andare in questo file /var/lib/tor/hidden_service/hostname



## NO! Non modificherò il client
se vuoi farti una botnet scrivitela da solo, questo è solo un semplice esempio a scopo informativo dovresti poter aggiungere la possibilità al client di scaricarsi snowflake/tor e connettendosi realmente ad un sito.onion esterno
ToDo,ButForU
