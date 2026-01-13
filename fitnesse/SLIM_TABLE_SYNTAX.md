# Syntaxe Correcte des Tables SLIM pour FitNesse

## Erreur Commune: "[0]" dans les Messages d'Erreur

Quand vous voyez une erreur comme:
```
No Method formationAvailable[0] in class uav.fixtures.FormationFixture
No Method responseContains[0] in class uav.fixtures.RestApiFixture
```

Le **[0]** signifie que SLIM cherche une méthode avec **0 arguments** (sans paramètres).

## Syntaxe SLIM Script Table

### Format Général
```
|script|fixture name              |
|action method                    |
|action method      |param1       |
|check|query method|param1 |value|
```

### Règle Important pour `check`

**Syntaxe avec Argument:**
```
|check|method name|argument|expected value|
```

- **Colonne 1**: `check` (keyword)
- **Colonne 2**: Nom de la méthode (camelCase devient "method name")
- **Colonne 3**: Argument passé à la méthode
- **Colonne 4**: Valeur attendue du résultat

## Exemples Corrects vs Incorrects

### Exemple 1: Vérifier qu'une Formation est Disponible

**❌ INCORRECT:**
```
|check|formation available        |triangle   |
```
**Interprétation SLIM**: Appeler `formationAvailable()` sans argument, vérifier que le résultat est "triangle"
**Erreur**: `No Method formationAvailable[0]`

**✅ CORRECT:**
```
|check|formation available|triangle|true|
```
**Interprétation SLIM**: Appeler `formationAvailable("triangle")`, vérifier que le résultat est "true"

### Exemple 2: Vérifier qu'un Champ Existe dans la Réponse

**❌ INCORRECT:**
```
|check|response contains|position    |
```
**Interprétation SLIM**: Appeler `responseContains()` sans argument, vérifier que le résultat est "position"
**Erreur**: `No Method responseContains[0]`

**✅ CORRECT:**
```
|check|response contains|position|true|
```
**Interprétation SLIM**: Appeler `responseContains("position")`, vérifier que le résultat est "true"

### Exemple 3: Vérifier un Status Code

**❌ INCORRECT:**
```
|check|status code         |
```
**Interprétation SLIM**: Appeler `statusCode()` et ne rien vérifier (syntaxe invalide)

**✅ CORRECT:**
```
|check|status code|200|
```
**Interprétation SLIM**: Appeler `statusCode()` sans argument, vérifier que le résultat est "200"

### Exemple 4: Appeler une Méthode avec Plusieurs Arguments

**Méthode Java:**
```java
public String updateDroneTarget(String droneId, double x, double y, double z)
```

**Syntaxe FitNesse:**
```
|script|drone fixture                     |
|update drone target|drone_1|50|100|25   |
|check|status code                  |200  |
```

**Interprétation SLIM**: Appeler `updateDroneTarget("drone_1", 50, 100, 25)`

## Convention de Nommage SLIM

SLIM convertit automatiquement les noms:

| Java (camelCase)       | FitNesse (space separated) |
|------------------------|----------------------------|
| `formationAvailable`   | `formation available`      |
| `responseContains`     | `response contains`        |
| `statusCode`           | `status code`              |
| `numberOfDrones`       | `number of drones`         |
| `simulationRunning`    | `simulation running`       |

## Patterns Courants dans Nos Tests

### 1. Action puis Vérification du Status
```
|script|swarm fixture       |
|start simulation           |
|check|status code    |200  |
```

### 2. Action avec Paramètre puis Vérification
```
|script|drone fixture             |
|get drone detail      |drone_1   |
|check|status code         |200   |
|check|drone id            |drone_1|
```

### 3. Vérifier l'Existence de Champs
```
|script|formation fixture                 |
|get current formation                    |
|check|response contains|formation_type      |true|
|check|response contains|separation_distance |true|
```

### 4. Vérifier des Valeurs Booléennes
```
|script|swarm fixture            |
|get swarm status                |
|check|simulation running |false |
|check|formation stable   |true  |
```

### 5. Vérifier des Valeurs Numériques
```
|script|drone fixture          |
|list drones                   |
|check|number of drones  |3    |
|check|status code       |200  |
```

### 6. Méthode avec Plusieurs Arguments
```
|script|mission fixture                      |
|create move to mission    |100|200|50      |
|check|status code                    |200   |
```

## Débogage des Erreurs SLIM

### Erreur: "No Method xxx[N]"

**[0]** = 0 arguments
- Problème: Vous n'avez pas fourni assez d'arguments
- Solution: Ajoutez les arguments nécessaires

**[1]** = 1 argument
- Problème: Vous avez fourni trop ou pas assez d'arguments
- Solution: Vérifiez la signature de la méthode Java

### Erreur: "Could not invoke constructor"
- Solution: Ajoutez un constructeur public sans arguments

### Erreur: "Method returned null"
- Solution: Assurez-vous que la méthode retourne toujours une valeur (même "")

## Vérification de la Syntaxe

Avant d'exécuter les tests, vérifiez:

1. ✅ Chaque `|check|` a au moins 3 colonnes (keyword, method, expected)
2. ✅ Si la méthode Java prend N arguments, fournissez N+2 colonnes total
3. ✅ Les noms de méthodes sont en "space separated" format
4. ✅ Les valeurs booléennes sont "true" ou "false" (pas TRUE/FALSE)
5. ✅ Les valeurs numériques sont des strings ("200" pas 200 dans la table)

## Résumé

| Situation | Syntaxe Correcte |
|-----------|------------------|
| Méthode sans argument | `\|check\|method name\|expected\|` |
| Méthode avec 1 argument | `\|check\|method name\|arg1\|expected\|` |
| Méthode avec 2 arguments | `\|check\|method name\|arg1\|arg2\|expected\|` |
| Action sans argument | `\|action method name\|` |
| Action avec arguments | `\|action method name\|arg1\|arg2\|...\|` |

## Fichiers Corrigés

Tous les tests ont été corrigés pour utiliser la syntaxe correcte:
- ✅ `DroneTests/content.txt`
- ✅ `SwarmTests/content.txt`
- ✅ `FormationTests/content.txt`
- ✅ `MissionTests/content.txt`

Les erreurs "[0]" ne devraient plus apparaître!

## Erreur Spécifique: Méthodes d'Action avec Arguments

### Problème: "No Method updateDroneTarget5025[3]"

Cette erreur indique que SLIM cherche une méthode avec le mauvais nombre d'arguments.

**❌ SYNTAXE INCORRECTE:**
```
|update drone target|drone_1|10|20|5| true |
```

**Problème**: Le `| true |` à la fin n'est pas valide dans une table script pour une méthode d'action. SLIM se confond et ne peut pas parser correctement les arguments.

**✅ SYNTAXE CORRECTE:**

Option 1 - Appeler la méthode puis vérifier le status:
```
|script|drone fixture                |
|update drone target|drone_1|10|20|5|
|check|status code            |200   |
```

Option 2 - Si vous voulez vérifier le résultat de la méthode elle-même:
```
|script|drone fixture                            |
|$result=|update drone target|drone_1|10|20|5  |
|check   |$result                         |OK   |
```

### Règle Importante: Méthodes d'Action vs Query

**Méthode d'Action** (effectue une opération):
```java
public String updateDroneTarget(String droneId, double x, double y, double z)
```

Syntaxe FitNesse:
```
|method name|arg1|arg2|arg3|arg4|
```
- Pas de vérification sur la même ligne
- Vérifiez le status ou résultat sur la ligne suivante

**Méthode Query** (retourne une valeur à vérifier):
```java
public String statusCode()
public String numberOfDrones()
```

Syntaxe FitNesse:
```
|check|method name|expected value|
|check|method name|arg1|expected value|
```

### Nombres d'Arguments dans les Erreurs

Le nombre entre crochets `[N]` indique le nombre d'arguments que SLIM a trouvé:

- `[0]` = 0 arguments trouvés
- `[1]` = 1 argument trouvé
- `[3]` = 3 arguments trouvés
- `[4]` = 4 arguments trouvés

**Exemple de l'erreur:**
```
No Method updateDroneTarget5025[3] in class
```

Signifie: SLIM a trouvé 3 arguments mais ne peut pas trouver la méthode. Le problème est que:
1. Le nom de la méthode a été mal interprété ("updateDroneTarget5025" au lieu de "updateDroneTarget")
2. Le nombre d'arguments est incorrect (3 au lieu de 4)

**Cause**: Syntaxe de table incorrecte avec des colonnes supplémentaires inattendues.

### Checklist pour Méthodes Multi-Arguments

1. ✅ Vérifiez le nombre d'arguments dans la signature Java
2. ✅ Comptez les colonnes dans la table FitNesse (après le nom de méthode)
3. ✅ Pas de colonnes supplémentaires après les arguments
4. ✅ Utilisez une ligne séparée pour vérifier les résultats

### Exemples Complets

**Méthode avec 4 arguments:**
```java
public String updateDroneTarget(String droneId, double x, double y, double z)
```

Table FitNesse:
```
|script|drone fixture                |
|update drone target|drone_1|50|100|25|
|check|status code            |200    |
```

**Méthode avec 3 arguments:**
```java
public String createMoveToMission(double x, double y, double z)
```

Table FitNesse:
```
|script|mission fixture              |
|create move to mission|100|200|50   |
|check|status code            |200   |
```

**Méthode avec 1 argument:**
```java
public String getDroneDetail(String droneId)
```

Table FitNesse:
```
|script|drone fixture        |
|get drone detail  |drone_1  |
|check|status code    |200   |
```

## Limitation SLIM: Comparaisons Non Supportées

### Problème: ensure avec Comparaisons (>=, <=, >, <)

**❌ SYNTAXE NON SUPPORTÉE:**
```
|ensure|number of missions|>= 3|
```

**Erreur**: `No Method numberOfMissions[1] in class`

**Cause**: SLIM ne supporte pas les opérateurs de comparaison (>=, <=, >, <) directement dans les tables. Il cherche une méthode avec un argument ">= 3" qui n'existe pas.

### Solutions Possibles

#### Solution 1: Note/Commentaire (Recommandé pour tests simples)
```
|script|mission fixture                     |
|create move to mission;   |10|20|5         |
|create move to mission;   |30|40|15        |
|create move to mission;   |50|60|25        |
|list missions                             |
|note|Verification: At least 3 missions should exist|
```

#### Solution 2: Méthode Helper Dédiée
Ajouter dans MissionFixture.java:
```java
public String numberOfMissionsAtLeast(int minimum) {
    int count = lastJsonResponse != null && lastJsonResponse.has("missions") 
        ? lastJsonResponse.getAsJsonArray("missions").size() 
        : 0;
    return String.valueOf(count >= minimum);
}
```

Utilisation:
```
|script|mission fixture                          |
|list missions                                  |
|check|number of missions at least|3     |true  |
```

#### Solution 3: Vérification Exacte (Si nombre connu)
```
|script|mission fixture              |
|list missions                       |
|check|number of missions      |3    |
```

### Keyword `note` en SLIM

Le keyword `note` permet d'ajouter des commentaires/notes dans les tests:
```
|note|This is a comment that will be displayed but not executed|
```

### Keyword `ensure` en SLIM

`ensure` attend une méthode qui retourne boolean/String true/false:
```
|ensure|method name that returns true/false|
```

**Bon usage:**
```java
public String simulationRunning() {
    return "true"; // ou "false"
}
```

```
|ensure|simulation running|
```

**Mauvais usage:**
```
|ensure|number of missions|>= 3|  ← SLIM pense que ">= 3" est un argument
```

### Comparaisons: Quand Utiliser Chaque Approche

| Scénario | Solution Recommandée |
|----------|---------------------|
| Test simple, valeur non critique | `note` commentaire |
| Valeur exacte connue | `check` avec valeur exacte |
| Comparaison nécessaire et réutilisable | Méthode helper dédiée |
| Condition complexe | Méthode helper avec logique |

### Autres Limitations SLIM

SLIM ne supporte pas nativement:
- Opérateurs mathématiques dans les tables (+, -, *, /)
- Expressions régulières directes
- Conditions if/else dans les tables
- Boucles for/while dans les tables

Pour ces cas, créez des méthodes Java dédiées qui encapsulent la logique.
