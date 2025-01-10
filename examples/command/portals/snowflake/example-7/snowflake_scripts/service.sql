!set variable_substitution=true
!variables

USE ROLE MSSQL_API_ROLE;
USE DATABASE MSSQL_API_DB;
USE WAREHOUSE MSSQL_API_WH;
USE SCHEMA MSSQL_API_SCHEMA;

DROP SERVICE IF EXISTS MSSQL_API_CLIENT;

CREATE SERVICE MSSQL_API_CLIENT
  IN COMPUTE POOL MSSQL_API_CP
  FROM SPECIFICATION
$$
    spec:
      endpoints:
      - name: http-endpoint
        port: 8080
        public: false
        protocol: HTTP
      - name: ockam-inlet
        port: 1443
        public: false
        protocol: TCP
      containers:
      - name: ockam-inlet
        image: /mssql_api_db/mssql_api_schema/mssql_api_repository/ockam
        env:
            OCKAM_DISABLE_UPGRADE_CHECK: true
            OCKAM_TELEMETRY_EXPORT: false
        args:
          - node
          - create
          - --foreground
          - --enrollment-ticket
          - "&ockam_ticket"
          - --configuration
          - |
            tcp-inlet:
              from: 0.0.0.0:1433
              via: mssql
              allow: mssql
      - name: http-endpoint
        image: /mssql_api_db/mssql_api_schema/mssql_api_repository/mssql_client
        env:
          SNOWFLAKE_WAREHOUSE: MSSQL_API_WH
          MSSQL_DATABASE: '&mssql_database'
          MSSQL_USER: '&mssql_user'
          MSSQL_PASSWORD: '&mssql_password'
        resources:
          requests:
            cpu: 0.5
            memory: 128M
          limits:
            cpu: 1
            memory: 256M
$$
MIN_INSTANCES=1
MAX_INSTANCES=1
EXTERNAL_ACCESS_INTEGRATIONS = (OCSP, OCKAM);
