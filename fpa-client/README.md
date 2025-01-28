# FPA Client

Design effort calculator using Function Point Analysis, based on information from the [International Function Point Users Group](https://ifpug.org).

## Uses

![GitHub](https://img.shields.io/github/license/LVRodrigues/fpa-management)

![Static Badge](https://img.shields.io/badge/angular-19-blue?logo=angular) 
![Static Badge](https://img.shields.io/badge/SAAS-yellow)
![Static Badge](https://img.shields.io/badge/NGXecharts-yellow)
![Static Badge](https://img.shields.io/badge/RSA-yellow)

## Publicar Nova Versão

Using the [GitFlow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) workflow, run the commands:

```bash
git flow release start <id>
npm run release-patch
git commit -a -m "Versão ???"
git flow release finish
```

The version is reported by three fields: [major, minor, patch]. The above command increments only the last field. To increment the minor field, use **npm run release**.