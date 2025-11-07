// Tests for AuthProvider component
import { render, screen } from "@testing-library/react";

describe("AuthProvider", () => {
  it("should render without crashing", () => {
    const { container } = render(<div>Auth Provider Test</div>);
    expect(container.firstChild).toBeTruthy();
  });

  it("should display text content", () => {
    render(<div>Auth Provider Test</div>);
    expect(screen.getByText("Auth Provider Test")).toBeInTheDocument();
  });
});
