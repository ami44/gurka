Feature: User
  Scenario: Create user
    When John creates an account
    Then an account named John is created

  Scenario: Cannot login with invalid credentials
    Given an existing user named John
    When John uses an invalid password
    Then a login failure error occurs

  Scenario: Edit User
    Given a logged in user named John
    When John edits a property of their account
    Then John's account is updated
  
  Scenario: Read user
    Given a user logged in named Jane
    And an existing user named John
    When Jane accesses John's user profile
    Then John's public profile is shown

  Scenario: Delete user
    Given a user logged in as an administrator
    When an account named John is selected for deletion
    Then John's account no longer exists

  Scenario: List all projects for a user
    Given a logged in user named John
    When a request to list all projects for the current user is received
    Then a list of all projects owned by the current user are shown