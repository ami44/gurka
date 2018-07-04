Feature: Steps
  Background:
    Given a feature with slug "test-feature" owned by John
    And a logged in user named John

  Scenario: Add a step to a feature
    When a step is submitted to the feature with slug "test-feature"
    Then a step is added to the feature with slug "test-feature"

  Scenario: Remove a step
    Given a step in the feature with slug "test-feature"
    When the current step is requested for deletion
    Then the step is removed from the current feature

  Scenario: Reorder steps
    Given I have a series of steps:
      | Step                      |
      | And the third test step   |
      | Then the last test step   |
      | Given the first test step |
      | When the second test step |
      | But the fourth test step  |
    When each step is moved according to the following:
      | From |  To  |
      |    3 |    1 |
      |    5 |    4 |
      |    2 |    3 |
      |    2 |    5 |
      |    4 |    2 |
    Then the expected order of each step after each move is:
      | A | B | C | D | E |
      | 1 | 3 | 5 | 2 | 4 |
      | 1 | 3 | 5 | 4 | 2 |
      | 1 | 5 | 3 | 4 | 2 |
      | 1 | 3 | 4 | 2 | 5 |
      | 1 | 2 | 3 | 4 | 5 |
