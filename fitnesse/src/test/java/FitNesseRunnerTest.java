import fitnesse.junit.FitNesseRunner;
import org.junit.runner.RunWith;

@RunWith(FitNesseRunner.class)
@FitNesseRunner.Suite("UavSwarmApi") // Nom de votre page FitNesse
@FitNesseRunner.FitnesseDir(".")       // Racine du wiki
@FitNesseRunner.OutputDir("target/fitnesse-reports")
public class FitNesseRunnerTest {
    // Cette classe reste vide, elle sert de point d'entrée pour Maven Surefire
}