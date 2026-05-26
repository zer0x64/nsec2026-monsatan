external_url 'http://gitlab.monsatan.ctf'
nginx['listen_addresses'] = ['0.0.0.0', '[::]']

gitlab_rails['gitlab_default_theme'] = 2

# https://docs.gitlab.com/omnibus/settings/memory_constrained_envs/

# Reduce resource usage
puma['worker_processes'] = 0
puma['worker_timeout'] = 120
sidekiq['concurrency'] = 5  # Could be reduced to 5, 10 is preferred
gitlab_kas['enable'] = false
gitlab_sshd['enable'] = false
gitaly['configuration'] = {
    concurrency: [
      {
        'rpc' => "/gitaly.SmartHTTPService/PostReceivePack",
        'max_per_repo' => 3,
      }, {
        'rpc' => "/gitaly.SSHService/SSHUploadPack",
        'max_per_repo' => 3,
      },
    ],
    hooks: {
      custom_hooks_dir: '/var/opt/gitlab/gitaly/custom_hooks',
  }
}

gitlab_rails['env'] = {
  'MALLOC_CONF' => 'dirty_decay_ms:1000,muzzy_decay_ms:1000',
  'GITLAB_RAILS_RACK_TIMEOUT' => 600
}
gitaly['env'] = {
  'MALLOC_CONF' => 'dirty_decay_ms:1000,muzzy_decay_ms:1000',
  'GITALY_COMMAND_SPAWN_MAX_PARALLEL' => '2'
}

gitlab_rails['monitoring_whitelist'] = ['::/0', '0.0.0.0/0']

# Disable monitoring
alertmanager['enable'] = false
gitlab_exporter['enable'] = false
gitlab_kas['enable'] = false
node_exporter['enable'] = false
postgres_exporter['enable'] = false
prometheus_monitoring['enable'] = false
prometheus['enable'] = false
puma['exporter_enabled'] = false
redis_exporter['enable'] = false
sidekiq['metrics_enabled'] = false
