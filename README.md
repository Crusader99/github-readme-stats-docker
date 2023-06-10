<p align="center">
  <img width="100px" src="https://res.cloudinary.com/anuraghazra/image/upload/v1594908242/logo_ccswme.svg" align="center" alt="GitHub Readme Stats" />
  <h2 align="center">GitHub Readme Stats for Docker</h2>
  <p align="center">
    <a href="https://hub.docker.com/r/crusaders/github-readme-stats-docker">
      <img alt="Docker Pulls" src="https://img.shields.io/docker/pulls/crusaders/github-readme-stats-docker" />
    </a>
    <a>
      <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/Crusader99/github-readme-stats-docker">
    </a>
    <a href="http://opensource.org/licenses/MIT">
      <img alt="License" src="https://img.shields.io/github/license/mdouchement/standardfile.svg" />
    </a>
  </p>
  <p align="center">Annoying "<a href="https://github.com/anuraghazra/github-readme-stats/issues/1471">maximum retries exceeded</a>" errors? Self-host private <a href="https://github.com/anuraghazra/github-readme-stats">github-readme-stats</a> instance on Docker!</p>
</p>

---

#### Features

- No "[maximum retries exceeded](https://github.com/anuraghazra/github-readme-stats)" errors due to self-hosting instead of using shared instance
- Images for arm/arm64 available for running on Raspberry Pi or other single board computers 
- Permits requests from different users than the configured `GITHUB_USER` to prevent server overload
- Includes statistics of your private repositories due to using your own `GITHUB_TOKEN`
- You don't have to pass your `GITHUB_TOKEN` to the 3rd-party hoster [Vercel](https://github.com/anuraghazra/github-readme-stats#on-vercel) (I don't trust them)


---

#### Setup

1. Create `GITHUB_TOKEN`:
   - Go to Settings, then Developer settings 
   - Personal access tokens (classic) &rarr; Generate new token (classic)
   - Select scopes: `repo:status`, `repo_deployment`, `public_repo`
   - Generate token

2. Start Docker container:
   - Environment variables have to be updated
   - The container will only allow API requests to the configured `GITHUB_USER`
   - This example starts the container on port `8080`
```
docker run -it -p 8080:80 -e GITHUB_USER=Crusader99 -e GITHUB_TOKEN=ghp_eTwj... crusaders/github-readme-stats-docker
```

3. Request custom profile stats:
   - Example: [http://localhost:8080/top-langs?username=Crusader99&layout=compact](http://localhost:8080/top-langs?username=Crusader99&layout=compact)
   - Note: The `username` parameter must match the configured `GITHUB_USER` environment variable
   - For more examples, refer to the documentation of [github-readme-stats](https://github-readme-stats.vercel.app/)

---

#### License

This project is licensed under the [MIT license](https://github.com/Crusader99/github-readme-stats-docker/blob/master/LICENSE).