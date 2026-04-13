import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import EmptyLibrary from './EmptyLibrary';

// Mock the api module
vi.mock('../../lib/api', () => ({
  api: {
    post: vi.fn(),
  },
}));

import { api } from '../../lib/api';

// Mock useNavigate
const mockNavigate = vi.fn();
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

describe('EmptyLibrary', () => {
  const mockOnUploadClick = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  const renderEmptyLibrary = () => {
    return render(
      <MemoryRouter>
        <EmptyLibrary onUploadClick={mockOnUploadClick} />
      </MemoryRouter>
    );
  };

  it('should render empty state message', () => {
    renderEmptyLibrary();

    expect(screen.getByText('Your library is empty')).toBeInTheDocument();
    expect(screen.getByText(/upload a pdf document/i)).toBeInTheDocument();
  });

  it('should render Try Sample button', () => {
    renderEmptyLibrary();

    expect(screen.getByRole('button', { name: /try a sample/i })).toBeInTheDocument();
  });

  it('should render Upload button', () => {
    renderEmptyLibrary();

    expect(screen.getByRole('button', { name: /upload a pdf/i })).toBeInTheDocument();
  });

  it('should call onUploadClick when upload button is clicked', async () => {
    const user = userEvent.setup();
    renderEmptyLibrary();

    await user.click(screen.getByRole('button', { name: /upload a pdf/i }));

    expect(mockOnUploadClick).toHaveBeenCalled();
  });

  it('should load sample and navigate on Try Sample click', async () => {
    const user = userEvent.setup();
    (api.post as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      project: { id: 'sample-123' },
    });

    renderEmptyLibrary();

    await user.click(screen.getByRole('button', { name: /try a sample/i }));

    await waitFor(() => {
      expect(api.post).toHaveBeenCalledWith('/api/samples/try');
      expect(mockNavigate).toHaveBeenCalledWith('/projects/sample-123');
    });
  });

  it('should show loading state while loading sample', async () => {
    const user = userEvent.setup();
    // Make API call take some time
    (api.post as ReturnType<typeof vi.fn>).mockImplementation(
      () => new Promise((resolve) => setTimeout(() => resolve({ project: { id: '123' } }), 100))
    );

    renderEmptyLibrary();

    await user.click(screen.getByRole('button', { name: /try a sample/i }));

    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('should show error message on sample load failure', async () => {
    const user = userEvent.setup();
    (api.post as ReturnType<typeof vi.fn>).mockRejectedValueOnce(new Error('API Error'));

    renderEmptyLibrary();

    await user.click(screen.getByRole('button', { name: /try a sample/i }));

    await waitFor(() => {
      expect(screen.getByText(/failed to load sample/i)).toBeInTheDocument();
    });
  });

  it('should show hint text', () => {
    renderEmptyLibrary();

    expect(screen.getByText(/try the sample to see how disona works/i)).toBeInTheDocument();
  });

  it('should disable Try Sample button while loading', async () => {
    const user = userEvent.setup();
    (api.post as ReturnType<typeof vi.fn>).mockImplementation(
      () => new Promise(() => {}) // Never resolves
    );

    renderEmptyLibrary();

    await user.click(screen.getByRole('button', { name: /try a sample/i }));

    expect(screen.getByRole('button', { name: /loading/i })).toBeDisabled();
  });
});
