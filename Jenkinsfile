// Jenkins Pipeline for UAV Swarm System
//
// Pipeline Structure:
// 1. Build - Compile the project
// 2. Test Software - Run software unit and integration tests
// 3. Test Acceptance - Run FitNesse acceptance tests
// 4. Test MBSE - Run MBSE traceability and validation tests
// 5. Report - Generate traceability and coverage reports
//
// Prerequisites:
// - Rust toolchain must be installed on the Jenkins agent
// - Maven and Java must be installed on the Jenkins agent
// - Jenkins user must have permissions to run cargo, maven, and java commands

pipeline {
    agent any

    options {
        // Keep builds for 30 days
        buildDiscarder(logRotator(daysToKeepStr: '30'))
        // Timeout after 2 hours
        timeout(time: 2, unit: 'HOURS')
        // Disable concurrent builds
        disableConcurrentBuilds()
        // Enable timestamps in console output
        timestamps()
    }

    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        RUST_VERSION = "1.92"
        // Git configuration
        GIT_DEPTH = '0'
    }

    stages {
        stage('Setup') {
            steps {
                script {
                    echo "🔍 Repository Information"
                    echo "  Repository URL: ${env.GIT_URL}"
                    echo "  Branch: ${env.GIT_BRANCH}"
                    echo "  Commit: ${env.GIT_COMMIT?.take(8)}"
                    echo "  Working directory: ${env.WORKSPACE}"
                    sh 'git --version'
                    sh 'git rev-parse --verify HEAD'
                    sh 'git status --short'
                    echo ""
                    echo "🦀 Rust Toolchain Information"
                    sh 'rustc --version'
                    sh 'cargo --version'

                    // Install cargo-nextest if not cached
                    sh '''
                        if ! command -v cargo-nextest &> /dev/null; then
                            echo "📦 Installing cargo-nextest..."
                            cargo install cargo-nextest --locked
                        else
                            echo "✅ cargo-nextest already installed (cached)"
                            cargo nextest --version
                        fi
                    '''
                }
            }
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // STAGE 1: BUILD
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        stage('Build') {
            parallel {
                stage('Validate Repository') {
                    steps {
                        script {
                            echo "🔍 Validating repository structure..."
                            sh '''
                                test -f Cargo.toml || (echo "❌ Cargo.toml not found" && exit 1)
                                test -d src || (echo "❌ src directory not found" && exit 1)
                                test -d tests || (echo "❌ tests directory not found" && exit 1)
                                echo "✅ Repository structure validated"
                                echo "📦 Checking Cargo project..."
                                cargo --version
                                cargo verify-project || (echo "❌ Invalid Cargo project" && exit 1)
                                echo "✅ Valid Cargo project"
                                echo "📊 Project statistics:"
                                echo "  Rust files: $(find src -name '*.rs' | wc -l)"
                                echo "  Test files: $(find tests -name '*.rs' | wc -l)"
                                echo "  Total lines: $(find src tests -name '*.rs' -exec cat {} \\; | wc -l)"
                            '''
                        }
                    }
                }

                /*
                stage('Check Formatting') {
                    when {
                        not { branch 'main' }
                    }
                    steps {
                        script {
                            catchError(buildResult: 'SUCCESS', stageResult: 'UNSTABLE') {
                                echo "🎨 Checking code formatting..."
                                sh 'rustup component add rustfmt'
                                sh 'cargo fmt -- --check'
                                echo "✅ Code formatting is correct"
                            }
                        }
                    }
                }

                stage('Clippy Lint') {
                    when {
                        not { branch 'main' }
                    }
                    steps {
                        script {
                            catchError(buildResult: 'SUCCESS', stageResult: 'UNSTABLE') {
                                echo "🔍 Running Clippy linter..."
                                sh 'rustup component add clippy'
                                sh 'cargo clippy -- -D warnings'
                                echo "✅ No clippy warnings"
                            }
                        }
                    }
                }
                */
            }
        }

        stage('Build Debug') {
            steps {
                script {
                    echo "🔨 Building project in debug mode..."
                    sh 'cargo build --verbose'
                    echo "✅ Debug build completed"
                }
            }
            post {
                success {
                    archiveArtifacts artifacts: 'target/debug/uav_swarm', fingerprint: true
                }
            }
        }

        stage('Build Release') {
           /* when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                }
            }*/
            steps {
                script {
                    echo "🔨 Building project in release mode..."
                    sh 'cargo build --release --verbose'
                    echo "✅ Release build completed"
                }
            }
            post {
                success {
                    archiveArtifacts artifacts: 'target/release/uav_swarm', fingerprint: true, onlyIfSuccessful: true
                }
            }
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // STAGE 2: SOFTWARE TESTS (Traditional Software Testing)
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        stage('Software Tests') {
            parallel {
                stage('Software Unit Tests') {
                    when {
                        not { tag pattern: '*', comparator: 'REGEXP' }
                    }
                    steps {
                        script {
                            echo "🧪 Running Software Unit Tests..."
                            sh '''
                                cargo nextest run --test software --profile ci -E "test(unit_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-software-unit.xml
                            '''
                            echo "✅ Software unit tests passed"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-software-unit.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-software-unit.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('Software Integration Tests') {
                    when {
                        not { tag pattern: '*', comparator: 'REGEXP' }
                    }
                    steps {
                        script {
                            echo "🔗 Running Software Integration Tests..."
                            sh '''
                                cargo nextest run --test software --profile ci -E "test(integration_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-software-integration.xml
                            '''
                            echo "✅ Software integration tests passed"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-software-integration.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-software-integration.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('All Software Tests') {
                    when {
                        anyOf {
                            branch 'main'
                            tag pattern: '*', comparator: 'REGEXP'
                        }
                    }
                    steps {
                        script {
                            echo "🧪 Running ALL Software Tests..."
                            sh '''
                                cargo nextest run --test software --profile ci --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-software-all.xml
                            '''
                            echo "✅ All software tests passed"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-software-all.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-software-all.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('Quick Feedback Tests') {
                    when {
                        allOf {
                            not { branch 'main' }
                            not { tag pattern: '*', comparator: 'REGEXP' }
                        }
                    }
                    steps {
                        script {
                            echo "⚡ Quick feedback - Running unit tests only..."
                            sh 'cargo nextest run --test software -E "test(unit_tests::)" --no-fail-fast'
                            echo "✅ Quick tests passed"
                        }
                    }
                }
            }
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // STAGE 3: ACCEPTANCE TESTS
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        stage('Acceptance Tests') {
            when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                }
            }
            steps {
                script {
                    echo "🧪 Running Acceptance Tests..."
                    sh '''
                        # Start the UAV server in background
                        cargo run -- serve --port 8080 > server.log 2>&1 &
                        SERVER_PID=$!
                        echo "Server started with PID: $SERVER_PID"

                        # Wait for server to be ready
                        echo "Waiting REST server is ready..."
                        for i in {1..30}; do
                            if curl -s http://localhost:8080/health > /dev/null; then
                                echo "Serveur prêt !"
                                break
                            fi
                            echo "Tentative $i/30..."
                            sleep 2
                        done

                        # Build FitNesse fixtures and run tests
                        cd fitnesse/fixtures && mvn clean install
                        cd ..
                        mvn clean test -Pfitnesse-tests

                        # Stop the server
                        echo "Stopping server (PID: $SERVER_PID)..."
                        kill $SERVER_PID || true

                        echo "✅ All Acceptance tests passed"
                    '''
                }
            }
            post {
                always {
                    junit testResults: 'fitnesse/target/surefire-reports/TEST-FitNesseRunnerTest.xml', allowEmptyResults: true
                    publishHTML([
                        allowMissing: true,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: 'fitnesse/target/fitnesse-reports/',
                        reportFiles: '**/*',
                        reportName: 'FitNesse Reports',
                        reportTitles: 'Detailed FitNesse Reports'
                    ])
                }
            }
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // STAGE 4: MBSE TRACEABILITY TESTS (Model-Based Systems Engineering)
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        stage('MBSE Tests') {
            when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                    changeRequest()
                }
            }
            parallel {
                stage('MBSE Component Mapping') {
                    steps {
                        script {
                            echo "🔗 Running MBSE Component Mapping Tests..."
                            sh '''
                                cargo nextest run --test mbse --profile ci -E "test(component_mapping_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-mbse-components.xml
                            '''
                            echo "✅ MBSE component mapping validated"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-mbse-components.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-mbse-components.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('MBSE Requirements Validation') {
                    steps {
                        script {
                            echo "📋 Running MBSE Requirements Validation Tests..."
                            sh '''
                                cargo nextest run --test mbse --profile ci -E "test(requirements_validation_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-mbse-requirements.xml
                            '''
                            echo "✅ All requirements validated"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-mbse-requirements.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-mbse-requirements.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('MBSE Safety Constraints') {
                    steps {
                        script {
                            echo "⚠️ Running MBSE Safety Constraints Tests..."
                            sh '''
                                cargo nextest run --test mbse --profile ci -E "test(safety_constraints_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-mbse-safety.xml
                            '''
                            echo "✅ All safety constraints verified"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-mbse-safety.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-mbse-safety.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('MBSE Traceability Matrix') {
                    steps {
                        script {
                            echo "📊 Running MBSE Traceability Matrix Tests..."
                            sh '''
                                cargo nextest run --test mbse --profile ci -E "test(traceability_matrix_tests::)" --verbose
                                mv target/nextest/ci/junit.xml target/nextest/junit-mbse-traceability.xml
                            '''
                            echo "✅ Traceability matrices validated"
                        }
                    }
                    post {
                        always {
                            junit testResults: 'target/nextest/junit-mbse-traceability.xml', allowEmptyResults: true
                            archiveArtifacts artifacts: 'target/nextest/junit-mbse-traceability.xml', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }
            }
        }

        stage('All MBSE Tests') {
            when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                }
            }
            steps {
                script {
                    echo "🔗 Running ALL MBSE Traceability Tests..."
                    sh '''
                        cargo nextest run --test mbse --profile ci --verbose
                        mv target/nextest/ci/junit.xml target/nextest/junit-mbse-all.xml
                    '''
                    echo "✅ All MBSE tests passed"
                }
            }
            post {
                always {
                    junit testResults: 'target/nextest/junit-mbse-all.xml', allowEmptyResults: true
                    archiveArtifacts artifacts: 'target/nextest/junit-mbse-all.xml', fingerprint: true, allowEmptyArchive: true
                }
            }
        }

        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        // STAGE 5: REPORTS (Generate comprehensive reports)
        // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        stage('Generate Reports') {
            when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                }
            }
            parallel {
                stage('Traceability Report') {
                    steps {
                        script {
                            echo "📊 Generating Complete Traceability Report..."
                            sh '''
                                cargo test --test mbse \
                                  traceability_matrix_tests::test_complete_traceability_report -- --nocapture \
                                  > traceability_report.txt 2>&1 || true
                            '''
                            echo "✅ Traceability report generated"
                            sh 'tail -100 traceability_report.txt'
                        }
                    }
                    post {
                        always {
                            archiveArtifacts artifacts: 'traceability_report.txt', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('Requirements Coverage Report') {
                    steps {
                        script {
                            echo "📋 Generating Requirements Coverage Report..."
                            sh '''
                                cargo test --test mbse \
                                  traceability_matrix_tests::test_requirements_coverage_analysis -- --nocapture \
                                  > requirements_coverage.txt 2>&1 || true
                            '''
                            echo "✅ Requirements coverage report generated"
                            sh 'tail -50 requirements_coverage.txt'
                        }
                    }
                    post {
                        always {
                            archiveArtifacts artifacts: 'requirements_coverage.txt', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }

                stage('Safety Constraints Report') {
                    steps {
                        script {
                            echo "⚠️ Generating Safety Constraints Report..."
                            sh '''
                                cargo test --test mbse \
                                  safety_constraints_tests::test_safety_constraints_documentation -- --nocapture \
                                  > safety_constraints_report.txt 2>&1 || true
                            '''
                            echo "✅ Safety constraints report generated"
                            sh 'cat safety_constraints_report.txt'
                        }
                    }
                    post {
                        always {
                            archiveArtifacts artifacts: 'safety_constraints_report.txt', fingerprint: true, allowEmptyArchive: true
                        }
                    }
                }
            }
        }

        stage('Test Summary') {
            when {
                anyOf {
                    branch 'main'
                    tag pattern: '*', comparator: 'REGEXP'
                }
            }
            steps {
                script {
                    echo "📊 Generating Test Summary Report..."
                    sh '''
                        cat > test_summary.txt <<EOF
UAV Swarm System - Test Summary
═══════════════════════════════════════════

Pipeline: ${BUILD_NUMBER}
Commit: ${GIT_COMMIT:0:8}
Branch: ${GIT_BRANCH}
Date: $(date)

SOFTWARE TESTS
─────────────────────────────────────────
EOF
                    '''

                    sh '''
                        # Extract test counts from JUnit reports
                        if [ -f target/nextest/junit-software-unit.xml ]; then
                            UNIT_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-software-unit.xml | head -1 || echo "N/A")
                            UNIT_FAILURES=$(grep -oP 'failures="\\K[0-9]+' target/nextest/junit-software-unit.xml | head -1 || echo "0")
                            echo "✅ Unit Tests: $UNIT_TESTS tests, $UNIT_FAILURES failures" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-software-integration.xml ]; then
                            INTEG_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-software-integration.xml | head -1 || echo "N/A")
                            INTEG_FAILURES=$(grep -oP 'failures="\\K[0-9]+' target/nextest/junit-software-integration.xml | head -1 || echo "0")
                            echo "✅ Integration Tests: $INTEG_TESTS tests, $INTEG_FAILURES failures" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-software-all.xml ]; then
                            TOTAL_SW=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-software-all.xml | head -1 || echo "N/A")
                            TOTAL_SW_FAIL=$(grep -oP 'failures="\\K[0-9]+' target/nextest/junit-software-all.xml | head -1 || echo "0")
                            echo "✅ Total Software Tests: $TOTAL_SW tests, $TOTAL_SW_FAIL failures" >> test_summary.txt
                        fi

                        cat >> test_summary.txt <<EOF

MBSE TRACEABILITY TESTS
─────────────────────────────────────────
EOF
                    '''

                    sh '''
                        if [ -f target/nextest/junit-mbse-components.xml ]; then
                            COMP_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-mbse-components.xml | head -1 || echo "N/A")
                            echo "✅ Component Mapping: $COMP_TESTS tests" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-mbse-requirements.xml ]; then
                            REQ_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-mbse-requirements.xml | head -1 || echo "N/A")
                            echo "✅ Requirements Validation: $REQ_TESTS tests" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-mbse-safety.xml ]; then
                            SAFE_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-mbse-safety.xml | head -1 || echo "N/A")
                            SAFE_FAIL=$(grep -oP 'failures="\\K[0-9]+' target/nextest/junit-mbse-safety.xml | head -1 || echo "0")
                            echo "✅ Safety Constraints: $SAFE_TESTS tests, $SAFE_FAIL failures" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-mbse-traceability.xml ]; then
                            TRACE_TESTS=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-mbse-traceability.xml | head -1 || echo "N/A")
                            echo "✅ Traceability Matrix: $TRACE_TESTS tests" >> test_summary.txt
                        fi

                        if [ -f target/nextest/junit-mbse-all.xml ]; then
                            TOTAL_MBSE=$(grep -oP 'tests="\\K[0-9]+' target/nextest/junit-mbse-all.xml | head -1 || echo "N/A")
                            TOTAL_MBSE_FAIL=$(grep -oP 'failures="\\K[0-9]+' target/nextest/junit-mbse-all.xml | head -1 || echo "0")
                            echo "✅ Total MBSE Tests: $TOTAL_MBSE tests, $TOTAL_MBSE_FAIL failures" >> test_summary.txt
                        fi

                        cat >> test_summary.txt <<EOF

OVERALL SUMMARY
═══════════════════════════════════════════
Status: Build ${BUILD_NUMBER} completed
All critical tests passed
EOF
                        cat test_summary.txt
                    '''
                }
            }
            post {
                always {
                    archiveArtifacts artifacts: 'test_summary.txt', fingerprint: true, allowEmptyArchive: true
                }
            }
        }

        stage('Release Validation') {
            when {
                tag pattern: '*', comparator: 'REGEXP'
            }
            steps {
                script {
                    echo "🎉 Validating Release..."
                    sh '''
                        echo "Checking all artifacts are present..."
                        test -f target/release/uav_swarm
                        test -f traceability_report.txt
                        test -f requirements_coverage.txt
                        test -f safety_constraints_report.txt
                        test -f test_summary.txt
                        echo "✅ Release validation complete"
                        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                        echo "🚀 Ready for release!"
                        cat test_summary.txt
                    '''
                }
            }
        }
    }

    post {
        always {
            // Clean workspace after build
            cleanWs(
                deleteDirs: true,
                disableDeferredWipeout: true,
                notFailBuild: true,
                patterns: [
                    [pattern: '.cargo', type: 'EXCLUDE'],
                    [pattern: 'target', type: 'EXCLUDE']
                ]
            )
        }
        success {
            echo '✅ Pipeline completed successfully!'
        }
        failure {
            echo '❌ Pipeline failed!'
        }
        unstable {
            echo '⚠️ Pipeline completed with warnings!'
        }
    }
}
