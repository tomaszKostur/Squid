Feature: Verify if server can respond with list of user orders

    Scenario: Server should return response with list of user orders

        Given User has account, API KEY, and API secret
        And User has no open orders
        When Request for list of orders is send
        Then There are not any orders in response body
        When User add some order
        And Request for list of orders is send
        Then There is description of one order in response body