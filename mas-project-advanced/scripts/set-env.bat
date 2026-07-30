@echo off
REM Script para configurar ambiente en Windows

if "%1"=="" (
    echo Uso: set-env.bat [development^|testing^|production]
    echo.
    echo Ejemplos:
    echo   set-env.bat development
    echo   set-env.bat testing
    echo   set-env.bat production
    exit /b 1
)

set ENV=%1

if "%ENV%"=="development" (
    copy .env.development .env
    echo ✅ Configurado para DESARROLLO
    echo 📊 Base de datos: acueducto_hato
    echo 🌐 Puerto: 8080
) else if "%ENV%"=="testing" (
    copy .env.testing .env
    echo ✅ Configurado para TESTING
    echo 📊 Base de datos: acueducto_hato_test
    echo 🌐 Puerto: 8081
) else if "%ENV%"=="production" (
    copy .env.production .env
    echo ✅ Configurado para PRODUCCIÓN
    echo ⚠️  Asegúrate de configurar las variables de entorno del sistema
    echo 📊 Revisa la configuración de base de datos
) else (
    echo ❌ Ambiente no válido: %ENV%
    echo Ambientes disponibles: development, testing, production
    exit /b 1
)

echo.
echo 🔄 Reinicia la aplicación para aplicar los cambios