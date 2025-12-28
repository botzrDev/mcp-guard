import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-blog',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>Blog</h1>
      <div class="blog-grid">
        <article class="blog-post">
          <div class="post-meta">December 26, 2025</div>
          <h2><a href="#">Introducing MCP Guard: Secure Your AI Infrastructure</a></h2>
          <p class="excerpt">
            As AI agents become more prevalent, the need for secure communication between agents and tools becomes critical. 
            Today, we are announcing MCP Guard, an open-source security proxy for the Model Context Protocol.
          </p>
          <a href="#" class="read-more">Read more →</a>
        </article>
      </div>
    </div>
  `,
  styles: [`
    .page-container {
      max-width: 800px;
      margin: 0 auto;
      padding: 4rem 2rem;
      margin-top: 80px;
    }

    h1 {
      font-size: 2.5rem;
      margin-bottom: 3rem;
      color: var(--text-primary, #111);
    }

    .blog-post {
      margin-bottom: 3rem;
      padding-bottom: 3rem;
      border-bottom: 1px solid var(--border-color, #eee);

      &:last-child {
        border-bottom: none;
      }
    }

    .post-meta {
      font-size: 0.875rem;
      color: var(--text-secondary, #666);
      margin-bottom: 0.5rem;
    }

    h2 {
      font-size: 1.75rem;
      margin-bottom: 1rem;
      
      a {
        text-decoration: none;
        color: var(--text-primary, #111);
        
        &:hover {
          color: var(--primary-color, #007bff);
        }
      }
    }

    .excerpt {
      line-height: 1.6;
      color: var(--text-secondary, #666);
      margin-bottom: 1.5rem;
    }

    .read-more {
      font-weight: 500;
      color: var(--primary-color, #007bff);
      text-decoration: none;
      
      &:hover {
        text-decoration: underline;
      }
    }
  `]
})
export class BlogComponent { }
