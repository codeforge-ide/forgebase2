-- Create sites table
CREATE TABLE IF NOT EXISTS sites (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    repository_url TEXT,
    default_branch VARCHAR(100) NOT NULL DEFAULT 'main',
    framework TEXT,
    build_command TEXT,
    output_directory TEXT,
    install_command TEXT,
    environment_variables JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sites_user_id ON sites(user_id);
CREATE INDEX idx_sites_slug ON sites(slug);
CREATE INDEX idx_sites_created_at ON sites(created_at);

-- Create deployments table
CREATE TABLE IF NOT EXISTS deployments (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    commit_sha VARCHAR(40),
    commit_message TEXT,
    branch VARCHAR(255) NOT NULL,
    status TEXT NOT NULL,
    build_logs TEXT,
    error_message TEXT,
    deployment_url TEXT NOT NULL,
    preview_url TEXT,
    build_duration_ms BIGINT,
    deployment_size_bytes BIGINT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deployed_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_deployments_site_id ON deployments(site_id);
CREATE INDEX idx_deployments_status ON deployments(status);
CREATE INDEX idx_deployments_branch ON deployments(branch);
CREATE INDEX idx_deployments_created_at ON deployments(created_at);

-- Create domains table
CREATE TABLE IF NOT EXISTS domains (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    domain VARCHAR(255) NOT NULL UNIQUE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    ssl_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ssl_cert TEXT,
    ssl_key TEXT,
    verification_token TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    verified_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_domains_site_id ON domains(site_id);
CREATE INDEX idx_domains_domain ON domains(domain);
CREATE INDEX idx_domains_is_verified ON domains(is_verified);
