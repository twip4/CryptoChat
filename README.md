# CryptoChat

### Description du Projet: Serveur de Chat Crypté en CLI (CryptoChat)

**Objectif**:
Développer une application de communication sécurisée et cryptée fonctionnant en ligne de commande, permettant à des utilisateurs de créer, rejoindre des salles de chat, et d'échanger des messages de manière sécurisée grâce au cryptage de bout en bout.

**Fonctionnalités Principales**:
1. **Création et gestion de salles de chat** : Les utilisateurs peuvent créer des salles de chat auxquelles d'autres utilisateurs peuvent se joindre en utilisant un identifiant unique de salle.
2. **Communication cryptée** : Tous les messages échangés entre les clients sont cryptés et décryptés à l'aide de clés uniques, assurant que seuls les participants peuvent lire les messages.
3. **Interface CLI** : Une interface utilisateur en ligne de commande simple et intuitive pour interagir avec le serveur de chat, envoyer des messages, et naviguer entre les salles de chat.
4. **Support multi-utilisateurs** : Capacité à gérer plusieurs utilisateurs simultanément grâce à une architecture de serveur asynchrone.
5. **Authentification** : Un système d'authentification basique pour identifier les utilisateurs et sécuriser l'accès aux salles de chat.

**Technologies et Concepts Utilisés**:
- **Langage de Programmation** : Rust, pour sa performance et sa sécurité mémoire.
- **Cryptographie** : Utilisation de bibliothèques de cryptographie pour le cryptage et le décryptage des messages.
- **Programmation Réseau** : Gestion des sockets pour la communication entre le serveur et les clients.
- **Programmation Asynchrone** : Pour gérer efficacement les multiples connexions client en parallèle.
- **Sérialisation/Desérialisation** : Pour le formatage et le parsing des messages et des données utilisateur.

**Architecture**:
- **Serveur** : Centralise la gestion des connexions, des salles de chat, et le relais des messages cryptés entre les clients. Ne déchiffre pas les messages, garantissant une confidentialité totale.
- **Client** : Application CLI permettant à l'utilisateur de se connecter au serveur, de participer à des salles de chat, d'envoyer et de recevoir des messages cryptés.

**Sécurité**:
- **Cryptage de bout en bout** : Assure que seuls les participants d'une salle de chat peuvent lire les messages, renforçant la confidentialité et la sécurité des communications.
- **Gestion sécurisée des clés** : Mécanismes de gestion des clés pour prévenir leur exposition ou leur compromission.

**Défi Technique**:
Le projet implique des défis tels que la mise en œuvre de cryptographie solide, la gestion des communications réseau de manière asynchrone et efficace, et la création d'une expérience utilisateur fluide via une interface CLI.