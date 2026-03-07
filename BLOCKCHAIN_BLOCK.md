# Structure d'un Bloc de la Blockchain Bitcoin

## Vue d'ensemble

Un bloc dans la blockchain Bitcoin est une structure de données qui contient un ensemble de transactions validées. Chaque bloc est lié au bloc précédent via un hash cryptographique, formant ainsi une chaîne de blocs (blockchain).

## Structure d'un Bloc Bitcoin

Un bloc Bitcoin contient les éléments suivants :

### 1. **En-tête du Bloc (Block Header)** - 80 bytes

L'en-tête du bloc contient les métadonnées essentielles du bloc :

#### 1.1. Version (4 bytes)
- **Utilité** : Indique la version du protocole Bitcoin utilisé pour créer le bloc
- **Valeur** : Entier sur 4 bytes (ex: `0x20000000` pour la version 2)
- **Fonction** : Permet d'implémenter des mises à jour du protocole (soft forks, hard forks)
- **Exemple** : `0x20000000` signifie que le bloc utilise les règles de la version 2

#### 1.2. Hash du Bloc Précédent (Previous Block Hash) - 32 bytes
- **Utilité** : Référence cryptographique au bloc précédent dans la chaîne
- **Valeur** : Hash SHA-256 double du bloc précédent (little-endian)
- **Fonction** : 
  - Crée la chaîne de blocs (chaque bloc pointe vers le précédent)
  - Garantit l'intégrité de la chaîne (modifier un bloc invalide tous les suivants)
  - Permet la validation de l'ordre chronologique
- **Exemple** : `00000000000000000008a3a41b85b8b29ad444def299fa217c9b9e77e83c8e2`

#### 1.3. Merkle Root (32 bytes)
- **Utilité** : Hash racine de l'arbre de Merkle contenant toutes les transactions du bloc
- **Valeur** : Hash SHA-256 double de la racine de l'arbre de Merkle
- **Fonction** :
  - Résume toutes les transactions du bloc en un seul hash
  - Permet la vérification rapide de l'inclusion d'une transaction (SPV - Simplified Payment Verification)
  - Garantit l'intégrité de toutes les transactions du bloc
- **Calcul** : 
  - Les transactions sont organisées en paires
  - Chaque paire est hashée ensemble
  - Le processus se répète jusqu'à obtenir un seul hash (la racine)
- **Exemple** : `ef1a8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5e8b5`

#### 1.4. Timestamp (4 bytes)
- **Utilité** : Horodatage Unix du moment où le bloc a été créé
- **Valeur** : Nombre de secondes écoulées depuis le 1er janvier 1970 (Unix epoch)
- **Fonction** :
  - Indique quand le bloc a été miné
  - Utilisé pour ajuster la difficulté du mining (tous les 2016 blocs)
  - Permet de suivre l'historique temporel de la blockchain
- **Exemple** : `0x5f8b2c3d` = 1603155005 = 19 octobre 2020

#### 1.5. Bits / Difficulté (4 bytes)
- **Utilité** : Représentation compacte de la cible de difficulté pour le Proof of Work
- **Valeur** : Format compact sur 4 bytes (ex: `0x1d00ffff`)
- **Fonction** :
  - Définit la difficulté du problème cryptographique à résoudre
  - Le mineur doit trouver un nonce tel que le hash du bloc soit inférieur à cette cible
  - S'ajuste automatiquement tous les 2016 blocs pour maintenir ~10 minutes par bloc
- **Format** : 
  - Premier byte : exposant
  - 3 bytes suivants : coefficient
  - Cible = coefficient × 2^(8 × (exposant - 3))

#### 1.6. Nonce (4 bytes)
- **Utilité** : Nombre aléatoire utilisé par les mineurs pour résoudre le Proof of Work
- **Valeur** : Entier non signé sur 4 bytes (0 à 4,294,967,295)
- **Fonction** :
  - Les mineurs modifient ce nombre pour trouver un hash valide
  - Le hash du bloc (avec ce nonce) doit être inférieur à la cible de difficulté
  - Preuve que le mineur a effectué un travail computationnel
- **Processus** :
  1. Le mineur assemble toutes les données du bloc
  2. Il essaie différents nonces (0, 1, 2, ...)
  3. Pour chaque nonce, il calcule le hash du bloc
  4. Si le hash < cible → bloc valide trouvé !
  5. Sinon, il incrémente le nonce et recommence

### 2. **Compteur de Transactions (Transaction Counter)** - Variable

- **Utilité** : Indique le nombre de transactions contenues dans le bloc
- **Format** : Variable-length integer (VarInt)
- **Fonction** : Permet au parser de savoir combien de transactions lire
- **Limite** : Environ 1-2 MB de transactions par bloc (limite de taille de bloc)

### 3. **Liste des Transactions** - Variable

Chaque bloc contient une liste de transactions, généralement :

#### 3.1. Transaction Coinbase (première transaction)
- **Utilité** : Transaction spéciale qui récompense le mineur
- **Contenu** :
  - Input : Aucun (crée de nouveaux bitcoins)
  - Output : Adresse du mineur + montant de la récompense (subsidy + fees)
- **Récompense** :
  - Subsidy : Récompense de base (actuellement 3.125 BTC, diminue de moitié tous les 210,000 blocs)
  - Fees : Somme de toutes les commissions des transactions du bloc
- **Fonction** : 
  - Émet de nouveaux bitcoins dans le système
  - Incite les mineurs à sécuriser le réseau
  - Contient parfois des messages (ex: "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks")

#### 3.2. Transactions Standard
- **Utilité** : Transfèrent des bitcoins d'une adresse à une autre
- **Structure** :
  - **Inputs** : Références aux outputs précédents (UTXO - Unspent Transaction Output)
  - **Outputs** : Nouvelles adresses et montants à transférer
  - **Scripts** : Scripts de déverrouillage (scriptSig) et de verrouillage (scriptPubKey)
- **Validation** :
  - Vérifie que les inputs existent et n'ont pas été dépensés
  - Vérifie que les signatures sont valides
  - Vérifie que la somme des inputs ≥ somme des outputs (fees = différence)

## Exemple de Bloc Bitcoin (Bloc Genesis)

Le premier bloc de Bitcoin (Bloc #0, créé par Satoshi Nakamoto) :

```
Version: 0x00000001
Previous Block Hash: 0000000000000000000000000000000000000000000000000000000000000000
Merkle Root: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
Timestamp: 0x495fab29 (1231006505 = 3 janvier 2009)
Bits: 0x1d00ffff
Nonce: 0x7c2bac1d

Transaction Coinbase:
  Message: "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"
  Output: 50 BTC vers l'adresse de Satoshi
```

## Processus de Mining

1. **Collecte des transactions** : Le mineur collecte des transactions non confirmées du mempool
2. **Vérification** : Il vérifie que toutes les transactions sont valides
3. **Construction du bloc** :
   - Crée la transaction coinbase
   - Construit l'arbre de Merkle avec toutes les transactions
   - Assemble l'en-tête du bloc
4. **Proof of Work** :
   - Essaie différents nonces
   - Calcule le hash SHA-256 double de l'en-tête
   - Continue jusqu'à trouver un hash < cible de difficulté
5. **Propagation** : Une fois trouvé, le bloc est diffusé sur le réseau
6. **Validation** : Les autres nœuds valident le bloc et l'ajoutent à leur chaîne

## Sécurité et Intégrité

### Protection contre la modification
- **Hash du bloc précédent** : Modifier un bloc invalide tous les blocs suivants
- **Merkle Root** : Modifier une transaction change le Merkle Root, invalidant le bloc
- **Proof of Work** : Nécessite une puissance de calcul énorme pour recalculer les nonces

### Consensus
- Les nœuds acceptent la chaîne la plus longue (avec le plus de travail cumulé)
- En cas de fork, la branche avec le plus de travail devient la chaîne principale
- Les transactions dans les blocs orphelins sont réintégrées au mempool

## Métriques Importantes

- **Taille moyenne d'un bloc** : ~1-2 MB
- **Nombre moyen de transactions par bloc** : ~2000-3000
- **Temps moyen entre les blocs** : ~10 minutes
- **Taille de l'en-tête** : 80 bytes (fixe)
- **Taille totale d'un bloc** : Variable (limite de ~4 MB avec SegWit)

## Conclusion

Un bloc Bitcoin est une structure sophistiquée qui combine :
- **Cryptographie** : Hash SHA-256, signatures ECDSA
- **Structures de données** : Arbre de Merkle, chaînage cryptographique
- **Consensus** : Proof of Work, validation distribuée
- **Économie** : Récompenses de mining, fees de transaction

Cette architecture garantit la sécurité, l'intégrité et la décentralisation du réseau Bitcoin.
