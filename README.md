Questo è un semplice progetto composto da 2 parti: una macchinina radiocomandata e il suo controller.

# Controller
Il controller è un Arduino Uno che legge continuamente la posizione del Joystick ad esso collegato, e la manda attraverso
l'antenna NRF24L01 alla macchina.

# Macchina
La macchina consiste in un Arduino Uno che controlla continuamente se l'antenna NRF24L01 ha ricevuto nuovi dati, e se li ha ricevuti
li analizza e usa il driver a ponte-H L298N per controllare il verso e la velocità dei 4 motori DC.