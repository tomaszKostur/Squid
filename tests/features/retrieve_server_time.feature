Feature: Verifying if server can retrieve current time

    Scenario: Server should return correct time

        Given Squid website is responding
        When Get request for server time is send
        Then Server time is returned in the response body