# Guide de Compatibilité SLIM pour FitNesse Fixtures

## Règle d'Or pour SLIM

**TOUTES les méthodes publiques appelées depuis FitNesse DOIVENT retourner String**

## Méthodes Affectées

### Méthodes d'Action
Les méthodes qui effectuent des opérations (GET, POST, PUT, DELETE):
```java
// ❌ NE PAS FAIRE
public void listDrones() { ... }

// ✅ FAIRE
public String listDrones() { 
    return executeGet("/api/drones"); 
}
```

### Méthodes de Query - Valeurs Numériques
```java
// ❌ NE PAS FAIRE
public int statusCode() { return 200; }
public int numberOfDrones() { return 3; }

// ✅ FAIRE
public String statusCode() { return "200"; }
public String numberOfDrones() { return "3"; }
```

### Méthodes de Query - Valeurs Booléennes
```java
// ❌ NE PAS FAIRE
public boolean responseContains(String field) { return true; }
public boolean simulationRunning() { return true; }

// ✅ FAIRE
public String responseContains(String field) { return "true"; }
public String simulationRunning() { return "true"; }
```

### Méthodes de Query - Valeurs Double
```java
// ❌ NE PAS FAIRE
public double maxSpeed() { return 5.0; }

// ✅ FAIRE
public String maxSpeed() { return "5.0"; }
```

## Pourquoi String ?

1. **SLIM gère mieux les String** - Reconnaissance automatique des méthodes
2. **Conversion automatique** - SLIM convertit String → int/boolean lors des comparaisons
3. **Pas de modification des tests** - Les tables FitNesse restent identiques
4. **Debug plus facile** - Valeurs lisibles dans les logs

## Exemples de Tests FitNesse qui Fonctionnent

### Test avec Vérification Numérique
```
|script|drone fixture       |
|list drones                |
|check|status code    |200  |    ← String "200" vs int 200 : OK
|check|number of drones|3   |    ← String "3" vs int 3 : OK
```

### Test avec Vérification Booléenne
```
|script|swarm fixture              |
|get swarm status                  |
|check|response contains|drones|true|  ← String "true" vs boolean : OK
|check|simulation running   |false|    ← String "false" vs boolean : OK
```

### Test avec Vérification Double
```
|script|drone fixture          |
|get drone detail      |drone_1|
|check|max speed         |5.0  |    ← String "5.0" vs double 5.0 : OK
```

## Liste Complète des Méthodes dans RestApiFixture

### Méthodes Publiques (toutes retournent String)
```java
public String statusCode()                           // Retourne status HTTP ("200", "404", etc.)
public String responseBody()                         // Retourne corps de la réponse
public String responseContains(String fieldName)     // Retourne "true" ou "false"
public String responseField(String fieldName)        // Retourne valeur du champ
```

### Méthodes Protégées (pour usage interne)
```java
protected String executeGet(String endpoint)         // Retourne "OK"
protected String executePost(String endpoint, ...)   // Retourne "OK"
protected String executePut(String endpoint, ...)    // Retourne "OK"
protected String executeDelete(String endpoint)      // Retourne "OK"

// Méthodes helper (gardent leur type pour usage interne)
protected int responseFieldAsInt(String fieldName)
protected boolean responseFieldAsBoolean(String fieldName)
```

## Pattern de Conversion

### Dans les Fixtures Enfants
```java
public class DroneFixture extends RestApiFixture {
    
    // Méthode d'action
    public String listDrones() {
        try {
            return executeGet("/api/drones");  // executeGet retourne String
        } catch (Exception e) {
            throw new RuntimeException("Failed: " + e.getMessage(), e);
        }
    }
    
    // Query numérique - conversion explicite
    public String numberOfDrones() {
        if (lastJsonResponse != null && lastJsonResponse.has("drones")) {
            int count = lastJsonResponse.getAsJsonArray("drones").size();
            return String.valueOf(count);  // int → String
        }
        return "0";
    }
    
    // Query booléenne - conversion explicite
    public String isActive() {
        boolean active = responseFieldAsBoolean("active");
        return String.valueOf(active);  // boolean → String
    }
    
    // Query double - conversion explicite
    public String maxSpeed() {
        if (lastJsonResponse != null && lastJsonResponse.has("max_speed")) {
            double speed = lastJsonResponse.get("max_speed").getAsDouble();
            return String.valueOf(speed);  // double → String
        }
        return "0.0";
    }
}
```

## Erreurs SLIM Communes et Solutions

### Erreur: "No Method xxxxx[0] in class"
**Cause**: La méthode retourne void, int, boolean, ou double au lieu de String

**Solution**: Changer le type de retour en String et convertir la valeur

### Erreur: "Could not invoke constructor"
**Cause**: Pas de constructeur public sans arguments

**Solution**: Ajouter `public MyFixture() { super(); }`

### Erreur: "The instance scriptTableActor.methodName. does not exist"
**Cause**: Fixture pas dans le classpath ou pas d'import

**Solution**: 
1. Ajouter `!|import|` avec le package
2. Vérifier le classpath dans le parent

## Checklist pour Nouvelle Fixture

- [ ] Constructeur public sans arguments
- [ ] Extends RestApiFixture
- [ ] TOUTES les méthodes publiques retournent String
- [ ] Exceptions wrappées dans RuntimeException
- [ ] Import ajouté dans la page de test FitNesse
- [ ] Build Maven réussi
- [ ] Test avec FitNesse

## Exemple de Fixture Complète et Conforme

```java
package uav.fixtures;

import com.google.gson.JsonObject;

public class ExampleFixture extends RestApiFixture {
    
    /**
     * Public constructor required by SLIM
     */
    public ExampleFixture() {
        super();
    }
    
    /**
     * Action: Get resource
     */
    public String getResource() {
        try {
            return executeGet("/api/resource");
        } catch (Exception e) {
            throw new RuntimeException("Failed: " + e.getMessage(), e);
        }
    }
    
    /**
     * Action: Create resource
     */
    public String createResource(String name) {
        try {
            JsonObject request = new JsonObject();
            request.addProperty("name", name);
            return executePost("/api/resource", gson.toJson(request));
        } catch (Exception e) {
            throw new RuntimeException("Failed: " + e.getMessage(), e);
        }
    }
    
    /**
     * Query: Get status (returns String, not int)
     */
    public String resourceStatus() {
        return responseField("status");
    }
    
    /**
     * Query: Get count (returns String, not int)
     */
    public String resourceCount() {
        int count = responseFieldAsInt("count");
        return count >= 0 ? String.valueOf(count) : "0";
    }
    
    /**
     * Query: Check if active (returns String, not boolean)
     */
    public String isActive() {
        return String.valueOf(responseFieldAsBoolean("active"));
    }
}
```

## Ressources

- FitNesse User Guide: http://fitnesse.org/FitNesse.UserGuide
- SLIM Protocol: http://fitnesse.org/FitNesse.UserGuide.WritingAcceptanceTests.SliM
- Type Conversions: http://fitnesse.org/FitNesse.UserGuide.WritingAcceptanceTests.SliM.ValueConversions
