Feature: Get User by ID

  Scenario Outline: Get User with invalid ID

    When I get the user with ID '<User ID>'
    Then I get a 404 response

    Examples:
      | User ID |
      | invalid |