import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { FileText, TrendingUp, Clock, Users } from 'lucide-react';
import { api } from '@/lib/api';
import { formatPrice } from '@/lib/utils';
import type { Template } from '@/types';

export function Marketplace() {
  const [templates, setTemplates] = useState<Template[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState<string>('');
  const [sort, setSort] = useState<string>('popular');

  useEffect(() => {
    loadTemplates();
  }, [category, sort]);

  const loadTemplates = async () => {
    try {
      setLoading(true);
      const result = await api.browseMarketplace({
        category: category || undefined,
        sort,
        page: 1,
        limit: 20,
      });
      setTemplates(result.templates);
    } catch (error) {
      console.error('Failed to load templates:', error);
    } finally {
      setLoading(false);
    }
  };

  const categories = [
    { value: '', label: 'All Categories' },
    { value: 'mvno', label: 'MVNO' },
    { value: 'saas', label: 'SaaS' },
    { value: 'nda', label: 'NDA' },
    { value: 'employment', label: 'Employment' },
    { value: 'service', label: 'Service Agreement' },
    { value: 'rental', label: 'Rental' },
  ];

  const sortOptions = [
    { value: 'popular', label: 'Most Popular', icon: TrendingUp },
    { value: 'recent', label: 'Recently Added', icon: Clock },
    { value: 'price', label: 'Price: Low to High', icon: FileText },
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-gray-900">Template Marketplace</h1>
          <p className="mt-2 text-gray-600">
            Browse and purchase contract templates for your business
          </p>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 py-8">
        {/* Filters */}
        <div className="flex flex-col sm:flex-row gap-4 mb-8">
          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Category
            </label>
            <select
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              className="input-field"
            >
              {categories.map((cat) => (
                <option key={cat.value} value={cat.value}>
                  {cat.label}
                </option>
              ))}
            </select>
          </div>

          <div className="flex-1">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Sort By
            </label>
            <select
              value={sort}
              onChange={(e) => setSort(e.target.value)}
              className="input-field"
            >
              {sortOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
        </div>

        {/* Templates Grid */}
        {loading ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {[...Array(6)].map((_, i) => (
              <div key={i} className="card animate-pulse">
                <div className="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                <div className="h-3 bg-gray-200 rounded w-full mb-2"></div>
                <div className="h-3 bg-gray-200 rounded w-2/3"></div>
              </div>
            ))}
          </div>
        ) : templates.length === 0 ? (
          <div className="text-center py-12">
            <FileText className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No templates found</h3>
            <p className="mt-1 text-sm text-gray-500">
              Try adjusting your filters or check back later.
            </p>
            <div className="mt-6">
              <Link to="/create-template" className="btn-primary">
                Create Your Own Template
              </Link>
            </div>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {templates.map((template) => (
              <Link
                key={template.id}
                to={`/templates/${template.id}`}
                className="card hover:shadow-lg transition-shadow cursor-pointer"
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-gray-900 mb-1">
                      {template.name}
                    </h3>
                    <p className="text-sm text-gray-500 line-clamp-2">
                      {template.description}
                    </p>
                  </div>
                  <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-primary-100 text-primary-800">
                    {template.category}
                  </span>
                </div>

                <div className="flex items-center gap-4 text-sm text-gray-600 mb-4">
                  <div className="flex items-center gap-1">
                    <Users className="h-4 w-4" />
                    <span>{template.usage_count} uses</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <span>⭐</span>
                    <span>{template.rating.toFixed(1)}</span>
                  </div>
                </div>

                <div className="flex items-center justify-between pt-4 border-t border-gray-200">
                  <span className="text-lg font-bold text-primary-600">
                    {formatPrice(template.price)}
                  </span>
                  <button className="btn-primary text-sm">Use Template</button>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
