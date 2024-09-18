# **EXPLORE AND EXPLOIT**

*Contributors*:
* Ethan Ford
* Nathan Freestone

## How to run this code!

1. install Rust ([guide](https://www.rust-lang.org/tools/install))

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. run the code

```bash
cargo run --bin part1
cargo run --bin part2
cargo run --bin part3
```
This will run the binaries for each individual part of the homework respectively. Graphs will appear as .png files in the root directory.

if you want it to run faster, you can try adding the `--release` tag to the command.



### **AN IMPORTANT NOTE**
The y-axis for each graph is total average reward.

## **Part 1: Epsilon-Greedy Algorithm in a Static Environment**

### *Epsilon-Greedy Algorithm*

* After implementing the epsilon-greedy algorithm, the optimal epsilon that converges the fastest, appeared to be 0.05:
  
  ![image](https://github.com/user-attachments/assets/778d920a-0ecc-454a-9019-b13387bb6e86)
  ![image](https://github.com/user-attachments/assets/f24521eb-5b9c-4c9d-9e0b-427f34e048c1)
  
* However, convergent rates varied among our tests. For example, 0.01 converged the fastest in this graph:

  ![image](https://github.com/user-attachments/assets/0d788b80-9de8-4f14-a35f-3dd41956b41b)

### *Thompson Sampling Algorithm*
* When comparing the optimal epsilon from the greedy algorithm to the thomson sampling, we see that the thomson sampling is slower to converge, but will eventually be faster than the greedy optimal epsilon.
* This is because we choose the action that maximizes benefit relative to a randomly drawn number, which in essence will be slower than epsilon at first, until we hone in on the maximum benefit which will result in faster convergence.

  ![image](https://github.com/user-attachments/assets/14c90d53-64cc-46fa-8c72-97f1baa935cf)
  ![image](https://github.com/user-attachments/assets/b3b42834-e5f4-431f-bfa5-f938fb3eab69)


## **Part 2: Exploring Epsilon in the Epsilon-Greedy Algorithm**

### *Epsilon Quenching Functions*

* The conversion rate for the linear quench starts to converge faster at a way later time a lot more consistently than the asymptotic or heavy asymptotic quenches so it is safe to rule it out completely.
* The asymptotic quench looks to be the best balance between exploration and exploitation, because of how we are reducing the epsilon. In the heavy asymptotic quench, we keep epsilon near 1 until we start to reach our last steps, meaning that we explore a lot more than we exploit. However, since the asymptotic quench reduces over the course of all of our steps, it's the best balance between all quenches.
  
  ![image](https://github.com/user-attachments/assets/abdc0ebb-bd37-4762-a748-3ce48cb781a8)

### *Modifying Exploration Strategy*

* As depicted in the graph below, it looks like the Explore Away from Best converges the fastest and is the most accurate. However, there was variance between methods as we conducted more tests, and whichever method is most fast and accurate is based purely on chance. Therefore, it doesn't seem like the explore method you use matters significantly, but the Exploring Away from the Best method is usually the safer bet.
  
  ![image](https://github.com/user-attachments/assets/aad09e01-b372-4d48-a3ba-b68170031bbf)


## **Part 3: Moving Bandits – Simulating Market Dynamics**

### *Reflection*

* Depending on the strategy you use, in this case, randomness will always tank your reward, however, when restarting Thomson at step 3000, we will always have the highest convergence by the 10,000th step. Compared to the standard approach, epsilon-greedy will not respond to the change in the market. The standard approach (Thomson) will immediately respond to the change in the market, and will rebound fairly well and decline at a stable rate. Thomson reset at step 3000 responds very well to the new market bumps, and declines at a stable rate as well. In essence, when thrown into an adaptable environment, you should not use epsilon greedy, but Thomson, as it reacts a lot better to adaptability.
  
  ![image](https://github.com/user-attachments/assets/682487be-e311-40d6-8d3f-31fa7b760dfa)

