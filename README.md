# Slack calculator Bot

[Deploy this function on flows.network](#deploy-slack-calculator-app-on-your-channel), and you will get a Slack bot that calculating the result of a mathematical formula.

<img width="856" alt="image" src="https://user-images.githubusercontent.com/45785633/226882668-faaf3550-ff68-4a3d-9509-d1482d22b803.png">

This is a simple example to show how flows.network works when the trigger and action both happens in Slack.

## Deploy Slack calculator App on your channel

To install this Slack calulator App, we will use [flows.network](https://flows.network/), a serverless platform enables you to deploy your own app quick and easy in three simple steps.

### Fork this repo and write your own code

Fork [this repo](https://github.com/flows-network/slack-calculator) and replace parameters in the red boxes in the image below with your own Slack workspace and channel.

![image](https://user-images.githubusercontent.com/45785633/226887619-e6bc05ce-d16b-48bb-9902-6c8f2ee68ec3.png)


### Deploy the code on flows.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the ChatGPT GitHub APP
3. Authenticate the [flows.network](https://flows.network/) to access the `slack-calculator` repo you just forked. 

![image](https://user-images.githubusercontent.com/45785633/226546523-93071359-b957-4653-a429-ab983ee9a078.png)

4. Click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

![image](https://user-images.githubusercontent.com/45785633/226888313-6e734828-f948-4e62-83fe-c26005aef445.png)

Here we can see, we need to configue one SaaS integration.

Click on the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on your Slack workspace. The workspace is the one you changed in the code.



After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the Slack calculator App goes live. Go ahead and calulate away in Slack!

![image](https://user-images.githubusercontent.com/45785633/226889061-a2443258-d8bf-4376-922a-dd85bd894d73.png)

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


## Others


To build locally

```
cargo build target wasm32-wasi --release
```
