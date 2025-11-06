// Load Test para Hodei Audit SDK
// Requiere k6: https://k6.io/
//
// Uso:
//   k6 run scripts/load-test-sdk.js

import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    stages: [
        { duration: '10s', target: 10 },   // Ramp up
        { duration: '30s', target: 50 },   // Stay at 50
        { duration: '30s', target: 100 },  // Ramp up to 100
        { duration: '1m', target: 100 },   // Stay at 100
        { duration: '30s', target: 200 },  // Ramp up to 200
        { duration: '1m', target: 200 },   // Stay at 200
        { duration: '30s', target: 0 },    // Ramp down
    ],
    thresholds: {
        http_req_duration: ['p(95)<1000'],  // 95% of requests under 1s
        http_req_failed: ['rate<0.01'],     // Error rate under 1%
    },
};

export default function() {
    // Test endpoints con diferentes patrones de paths

    // 1. Policy stores (verified-permissions)
    let responses = http.batch([
        ['GET', 'http://app:50051/v1/policy-stores', null, { tags: { name: 'policy_stores_list' } }],
        ['POST', 'http://app:50051/v1/policy-stores', JSON.stringify({ name: 'test' }), { tags: { name: 'policy_stores_create' } }],
        ['GET', 'http://app:50051/v1/policy-stores/default', null, { tags: { name: 'policy_stores_get' } }],
    ]);

    // Verificar responses
    check(responses[0], {
        'policy stores list status is 200': (r) => r.status === 200,
        'policy stores list has no errors': (r) => r.status < 400,
    });

    check(responses[1], {
        'policy stores create status is 201': (r) => r.status === 201,
        'policy stores create has no errors': (r) => r.status < 400,
    });

    check(responses[2], {
        'policy stores get status is 200': (r) => r.status === 200,
        'policy stores get has no errors': (r) => r.status < 400,
    });

    // 2. Authorization checks
    let authResponse = http.post(
        'http://app:50051/v1/authorize',
        JSON.stringify({
            principal: 'User:alice',
            action: 'read',
            resource: 'Document:123',
        }),
        {
            headers: { 'Content-Type': 'application/json' },
            tags: { name: 'authorize' }
        }
    );

    check(authResponse, {
        'authorize status is 200': (r) => r.status === 200,
        'authorize has no errors': (r) => r.status < 400,
        'authorize response time OK': (r) => r.timings.duration < 1000,
    });

    // 3. API endpoints
    let apiResponse = http.batch([
        ['GET', 'http://app:50051/api/v1/users', null, { tags: { name: 'api_users_list' } }],
        ['GET', 'http://app:50051/api/v1/users/123', null, { tags: { name: 'api_users_get' } }],
        ['POST', 'http://app:50051/api/v1/users', JSON.stringify({ name: 'Bob' }), { tags: { name: 'api_users_create' } }],
    ]);

    check(apiResponse[0], {
        'api users list status is 200': (r) => r.status === 200,
    });

    check(apiResponse[1], {
        'api users get status is 200': (r) => r.status === 200,
    });

    check(apiResponse[2], {
        'api users create status is 201': (r) => r.status === 201,
    });

    // 4. Auth endpoints
    let authEndpoints = http.batch([
        ['POST', 'http://app:50051/v1/auth/login', JSON.stringify({ username: 'alice', password: 'secret' }), { tags: { name: 'auth_login' } }],
        ['POST', 'http://app:50051/v1/auth/logout', null, { tags: { name: 'auth_logout' } }],
    ]);

    check(authEndpoints[0], {
        'auth login status is 200': (r) => r.status === 200,
    });

    check(authEndpoints[1], {
        'auth logout status is 200': (r) => r.status === 200,
    });

    // Sleep entre iterations
    sleep(1);
}

export function handleSummary(data) {
    return {
        'reports/load-test-summary.html': htmlReport(data),
        'reports/load-test-summary.json': JSON.stringify(data),
    };
}

function htmlReport(data) {
    return `
<!DOCTYPE html>
<html>
<head>
    <title>Load Test Report - Hodei Audit SDK</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1 { color: #333; }
        .metric { margin: 10px 0; padding: 10px; background: #f5f5f5; border-radius: 5px; }
        .good { color: green; }
        .bad { color: red; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #4CAF50; color: white; }
    </style>
</head>
<body>
    <h1>📊 Load Test Report - Hodei Audit SDK</h1>

    <div class="metric">
        <h2>Summary</h2>
        <p><strong>Total Requests:</strong> ${data.root_group.checks.length || 'N/A'}</p>
        <p><strong>Data Received:</strong> ${(data.root.group_counts?.data_received / 1024 / 1024).toFixed(2)} MB</p>
        <p><strong>Data Sent:</strong> ${(data.root.group_counts?.data_sent / 1024 / 1024).toFixed(2)} MB</p>
        <p><strong>Duration:</strong> ${(data.root.group.duration / 1000).toFixed(2)} seconds</p>
    </div>

    <h2>Thresholds</h2>
    <table>
        <tr>
            <th>Metric</th>
            <th>Target</th>
            <th>Actual</th>
            <th>Status</th>
        </tr>
        <tr>
            <td>HTTP Request Duration (p95)</td>
            <td>&lt; 1000ms</td>
            <td>${data.metrics.http_req_duration.values['p(95)']?.toFixed(2) || 'N/A'}ms</td>
            <td class="${data.metrics.http_req_duration.values['p(95)'] < 1000 ? 'good' : 'bad'}">
                ${data.metrics.http_req_duration.values['p(95)'] < 1000 ? '✅ PASS' : '❌ FAIL'}
            </td>
        </tr>
        <tr>
            <td>Error Rate</td>
            <td>&lt; 1%</td>
            <td>${(data.metrics.http_req_failed.values.rate * 100).toFixed(2)}%</td>
            <td class="${data.metrics.http_req_failed.values.rate < 0.01 ? 'good' : 'bad'}">
                ${data.metrics.http_req_failed.values.rate < 0.01 ? '✅ PASS' : '❌ FAIL'}
            </td>
        </tr>
    </table>

    <h2>Metrics</h2>
    <div class="metric">
        <p><strong>HTTP Requests:</strong> ${data.metrics.http_reqs.values.count}</p>
        <p><strong>HTTP Request Rate:</strong> ${data.metrics.http_reqs.values.rate.toFixed(2)} req/s</p>
        <p><strong>Average Response Time:</strong> ${data.metrics.http_req_duration.values.avg?.toFixed(2) || 'N/A'}ms</p>
        <p><strong>Min Response Time:</strong> ${data.metrics.http_req_duration.values.min?.toFixed(2) || 'N/A'}ms</p>
        <p><strong>Max Response Time:</strong> ${data.metrics.http_req_duration.values.max?.toFixed(2) || 'N/A'}ms</p>
        <p><strong>Median Response Time:</strong> ${data.metrics.http_req_duration.values.med?.toFixed(2) || 'N/A'}ms</p>
    </div>

    <h2>Checks</h2>
    <table>
        <tr>
            <th>Check</th>
            <th>Passed</th>
            <th>Failed</th>
            <th>Pass Rate</th>
        </tr>
        ${Object.entries(data.root.checks).map(([name, check]) => `
            <tr>
                <td>${name}</td>
                <td>${check.passes}</td>
                <td>${check.fails}</td>
                <td class="${check.passes / (check.passes + check.fails) > 0.95 ? 'good' : 'bad'}">
                    ${((check.passes / (check.passes + check.fails)) * 100).toFixed(2)}%
                </td>
            </tr>
        `).join('')}
    </table>

    <h2>Conclusion</h2>
    <p>El SDK de auditoría debe mantener un overhead < 1ms por request y una tasa de error < 1%.</p>
    <p>El batch processing debe reducir el network overhead en ~99% (1 call / 100 requests).</p>
</body>
</html>
    `;
}
