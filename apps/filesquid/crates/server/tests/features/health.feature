Feature: Mailbox health
  Scenario: The hosted process answers
    When a client GET /health
    Then the status is ok and the product is filesquid
