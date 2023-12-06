Questo è un semplice progetto composto da 2 parti: una macchinina radiocomandata e il suo controller.

# Controller
Il controller è un Arduino Uno che legge continuamente la posizione del Joystick ad esso collegato, e la manda attraverso
l'antenna NRF24L01 alla macchina.

# Macchina
La macchina consiste in un Arduino Uno che controlla continuamente se l'antenna NRF24L01 ha ricevuto nuovi dati, e se li ha ricevuti
li analizza e usa il driver a ponte-H L298N per controllare il verso e la velocità dei 4 motori DC.

# Video
https://github.com/Angelo13C/Macchina-RC/assets/55251189/34a6e3b5-b490-441e-b644-98ef99e5eeaa
