Feature: Verifying if server coretly resturns trading pair XBT/USD

    Scenario: Server should return trading pair XBT/USD
        Given Squid website is responding
        When Get request for XBT/USD trading pair is send
        Then All trading pair informations are in response body

    Scenario: Server should return traiding pair XBT/USD, but response is limited to leverage
        Given Squid website is responding
        When Get request for XBT/USD trading pair, limited to leverage info is send
        Then Leverage info for trading pair is in response body

    Scenario: Server should return traiding pair XBT/USD, but response is limited to fees
        Given Squid website is responding
        When Get request for XBT/USD trading pair, limited to fees info is send
        Then Fees info for trading pair is in response body

    Scenario: Server should return traiding pair XBT/USD, but response is limited to margin
        Given Squid website is responding
        When Get request for XBT/USD trading pair, limited to margin info is send
        Then Margin info for trading pair is in response body

    Scenario: Server should return traiding pair XBT/USD, and request info has info value
        Given Squid website is responding
        When Get request for XBT/USD trading pair, with info query value info is send
        Then All trading pair informations are in response body

    Scenario: Server should return error on invalid info value
        Given Squid website is responding
        When Get request for XBT/USD trading pair, with info invalid value is send
        Then Response body contains Invalid argumants error