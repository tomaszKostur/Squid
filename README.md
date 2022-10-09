Dear interviewers


I think I did enough for the interview task.
There are probably some improvements that could be made to this code, as well as the addition of more test cases.
However, many philosophers claimed that perfection existed only in theory.


My final thoughts, which I'd like to share, are:
* I'm new to the Gherkin language, so I can't judge if phrases in feature files are done idiomatically.
* I had some trouble with authentication, HMAC was something new for me.
I noticed that in private messages _nonce_ has to be in the first place in the request body.
If not I get InvalidKey error from server.
That's why I had to use an IndexMap crate to create the request body instead of a HashMap.
I'm not sure if this is supposed to work like that.
* I can do private requests using hmac, but there is no need to add any "otp" in my requests.
I get mail about how my two-factor authentication is enabled, but probably there is still something missing in my account configuration.
* My idea for checking the correctness of response bodies was to:
* Deserialize it to structure. That should be proof that no key is missing and all keys have the right type.
* Implement PartialEq with a bonded structure for a detailed check of the values inside. A more detailed explanation is in helper_structs.rs
I'm not sure if this is idiomatic way of checking output from server, maybe with some model of output the checking mechanism might be implemented more generic.
* Reaport of tests is just cucumber output in stdout. Some improvement could be generate sth preattier, eg. Allure reaport


To run the tests in Docker:


fill API_KEY and API_SEC fields in Dockerfile


docker build --tag=squid .
docker run --rm squid

Note that to successfully pass "get_user_orders.feature" it is needed to have at least 0.011 ETH available on the account.



Best Regards, Tomasz Kostur