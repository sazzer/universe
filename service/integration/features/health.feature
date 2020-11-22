Feature: Healthchecks

  Scenario: Healthchecks
    When I get the system health
    Then I get a 200 response
    And I get an "application/json" content of
      """
      {
        "components": {
          "db": {
            "healthy": true
          }
        },
        "healthy": true
      }
      """
