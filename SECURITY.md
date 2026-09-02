<img width="200" alt="Aster" src="https://raw.githubusercontent.com/Aster-Privacy/.github/main/profile/aster_logo.png" />

# Security Policy

## Reporting a vulnerability

**Do not open a public GitHub issue for a security vulnerability.**

Send your report to security@astermail.org.

You can also submit reports through our vulnerability disclosure program on Bugcrowd: https://bugcrowd.com/engagements/aster-privacy-vdpc

We read reports within 48 hours and resolve critical vulnerabilities within seven days. We keep you updated throughout the process.

## Scope

This policy covers all Aster products and infrastructure, including every repository under github.com/Aster-Privacy.

For this repository, the reports we care about most are ones where the bot leaks its own credentials, where a status page or Discord message can drive it into unintended behavior, or where its dependencies carry a known vulnerability.

## Running the bot safely

- Keep `DISCORD_TOKEN` and `API_TOKEN` in a `.env` file or your process manager's environment. Never commit them.
- Give the bot a Better Stack API token with the narrowest access your team allows. It only reads status page resources.
- Give the bot the **Send Messages** and **Embed Links** permissions and nothing more.
- Announcements repeat the description text from your status page. Anyone who can publish to your status page can therefore post text in your Discord channel, so treat status page access as equivalent to posting access in the updates channel.

## Safe harbor

We never pursue legal action against researchers who:

- Report vulnerabilities in good faith
- Do not access, modify, or exfiltrate user data
- Do not disrupt service availability or degrade user experience
- Allow us a reasonable timeframe to respond before public disclosure

## Coordinated disclosure

We follow coordinated disclosure. Give us adequate time to patch a vulnerability before you publish. We are happy to credit you publicly if you want. Say so in your report.
